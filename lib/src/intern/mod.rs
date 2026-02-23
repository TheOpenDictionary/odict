//! General-purpose value interning for rkyv.

mod interner;
mod polyfill;
mod se;

extern crate alloc;

use alloc::boxed::Box;
use core::{
    alloc::Layout, borrow::Borrow, error::Error, fmt, marker::PhantomData, ops::Deref, ptr::NonNull,
};

use rkyv::{
    rancor::{fail, Fallible, ResultExt as _, Source, Strategy},
    rc::{ArchivedRc, Flavor, RcResolver},
    ser::{sharing::SharingState, Allocator, Positional, Sharing, Writer},
    traits::LayoutRaw,
    with::{ArchiveWith, DeserializeWith, SerializeWith},
    Archive, ArchiveUnsized, Deserialize, DeserializeUnsized, Place, Serialize, SerializeUnsized,
};

pub use self::interner::*;
pub use self::se::*;

/// A shared value interning strategy.
///
/// This trait is required to use [`Intern`] and [`DerefIntern`].
pub trait Interning<T: ?Sized, E = <Self as Fallible>::Error> {
    /// Starts interning the given value.
    fn start_interning(&mut self, value: &T) -> SharingState;

    /// Finishes interning the given value.
    ///
    /// Returns an error if the value was not pending.
    fn finish_interning(&mut self, value: &T, pos: usize) -> Result<(), E>;
}

#[derive(Debug)]
struct CyclicInternedValueError;

impl fmt::Display for CyclicInternedValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "encountered cyclic shared pointers while interning")
    }
}

impl Error for CyclicInternedValueError {}

/// Helper methods for [`Interning`].
pub trait InterningExt<T: ?Sized, E>: Interning<T, E> {
    /// Interns and serializes a value.
    ///
    /// Returns the position of the interned value.
    fn serialize_interned(&mut self, value: &T) -> Result<usize, <Self as Fallible>::Error>
    where
        Self: Fallible<Error = E>,
        E: Source,
        T: SerializeUnsized<Self>,
    {
        match self.start_interning(value) {
            SharingState::Started => {
                let pos = value.serialize_unsized(self)?;
                self.finish_interning(value, pos)?;
                Ok(pos)
            }
            SharingState::Pending => fail!(CyclicInternedValueError),
            SharingState::Finished(pos) => Ok(pos),
        }
    }
}

impl<S, T, E> InterningExt<T, E> for S
where
    S: Interning<T, E> + ?Sized,
    T: ?Sized,
{
}

/// The flavor type for interned values.
pub struct InternFlavor;

impl Flavor for InternFlavor {
    const ALLOW_CYCLES: bool = false;
}

/// A wrapper that pools copies of the same value to reduce serialized size.
///
/// # Example
///
/// ```
/// use rkyv::Archive;
/// use rkyv_intern::Intern;
///
/// #[derive(Archive)]
/// struct Example {
///     #[rkyv(with = Intern)]
///     name: String,
/// }
/// ```
#[derive(Debug)]
pub struct Intern;

impl<T: Archive> ArchiveWith<T> for Intern {
    type Archived = ArchivedRc<T::Archived, InternFlavor>;
    type Resolver = RcResolver;

    fn resolve_with(field: &T, resolver: Self::Resolver, out: Place<Self::Archived>) {
        ArchivedRc::resolve_from_ref(field, resolver, out);
    }
}

impl<T, S> SerializeWith<T, S> for Intern
where
    T: Serialize<S>,
    S: Interning<T> + Writer + Fallible + ?Sized,
    S::Error: Source,
{
    fn serialize_with(
        field: &T,
        serializer: &mut S,
    ) -> Result<Self::Resolver, <S as Fallible>::Error> {
        Ok(RcResolver::from_pos(serializer.serialize_interned(field)?))
    }
}

impl<T, D> DeserializeWith<ArchivedRc<T::Archived, InternFlavor>, T, D> for Intern
where
    T: Archive,
    T::Archived: Deserialize<T, D>,
    D: Fallible + ?Sized,
{
    fn deserialize_with(
        field: &ArchivedRc<T::Archived, InternFlavor>,
        deserializer: &mut D,
    ) -> Result<T, <D as Fallible>::Error> {
        field.deserialize(deserializer)
    }
}

/// A wrapper that shares copies of the same `Deref`-ed value to reduce
/// serialized size.
///
/// # Example
///
/// ```
/// use rkyv::Archive;
/// use rkyv_intern::DerefIntern;
///
/// #[derive(Archive)]
/// struct Example {
///     #[rkyv(with = DerefIntern)]
///     name: String,
/// }
/// ```
#[allow(dead_code)]
#[derive(Debug)]
pub struct DerefIntern;

impl<T: Deref> ArchiveWith<T> for DerefIntern
where
    T::Target: ArchiveUnsized,
{
    type Archived = ArchivedRc<<T::Target as ArchiveUnsized>::Archived, InternFlavor>;
    type Resolver = RcResolver;

    fn resolve_with(field: &T, resolver: Self::Resolver, out: Place<Self::Archived>) {
        ArchivedRc::resolve_from_ref(field.deref(), resolver, out);
    }
}

impl<T, S> SerializeWith<T, S> for DerefIntern
where
    T: Deref,
    T::Target: SerializeUnsized<S>,
    S: Interning<T::Target> + Writer + Fallible + ?Sized,
    S::Error: Source,
{
    fn serialize_with(
        field: &T,
        serializer: &mut S,
    ) -> Result<Self::Resolver, <S as Fallible>::Error> {
        Ok(RcResolver::from_pos(serializer.serialize_interned(field)?))
    }
}

impl<T, D> DeserializeWith<ArchivedRc<<T::Target as ArchiveUnsized>::Archived, InternFlavor>, T, D>
    for DerefIntern
where
    T: Deref + From<Box<T::Target>>,
    T::Target: ArchiveUnsized + LayoutRaw,
    <T::Target as ArchiveUnsized>::Archived: DeserializeUnsized<T::Target, D>,
    D: Fallible + ?Sized,
    D::Error: Source,
{
    fn deserialize_with(
        field: &ArchivedRc<<T::Target as ArchiveUnsized>::Archived, InternFlavor>,
        deserializer: &mut D,
    ) -> Result<T, <D as Fallible>::Error> {
        let metadata = field.get().deserialize_metadata();
        let layout = T::Target::layout_raw(metadata).into_error()?;
        let data_address = if layout.size() > 0 {
            unsafe { alloc::alloc::alloc(layout) }
        } else {
            polyfill::dangling(&layout).as_ptr()
        };

        let out = rkyv::ptr_meta::from_raw_parts_mut(data_address.cast(), metadata);

        unsafe {
            field.get().deserialize_unsized(deserializer, out)?;
        }
        unsafe { Ok(T::from(Box::from_raw(out))) }
    }
}

/// A wrapper that shares copies of the same `Borrow`-ed value to reduce
/// serialized size.
///
/// # Example
///
/// ```
/// use rkyv::Archive;
/// use rkyv_intern::BorrowIntern;
///
/// #[derive(Archive)]
/// struct Example {
///     #[rkyv(with = BorrowIntern<str>)]
///     name: String,
/// }
/// ```
#[allow(dead_code)]
#[derive(Debug)]
pub struct BorrowIntern<B: ?Sized> {
    _phantom: PhantomData<B>,
}

impl<T, B> ArchiveWith<T> for BorrowIntern<B>
where
    T: Borrow<B>,
    B: ArchiveUnsized + ?Sized,
{
    type Archived = ArchivedRc<B::Archived, InternFlavor>;
    type Resolver = RcResolver;

    fn resolve_with(field: &T, resolver: Self::Resolver, out: Place<Self::Archived>) {
        ArchivedRc::resolve_from_ref(field.borrow(), resolver, out);
    }
}

impl<T, S, B> SerializeWith<T, S> for BorrowIntern<B>
where
    T: Borrow<B>,
    S: Interning<B> + Writer + Fallible + ?Sized,
    S::Error: Source,
    B: SerializeUnsized<S> + ?Sized,
{
    fn serialize_with(
        field: &T,
        serializer: &mut S,
    ) -> Result<Self::Resolver, <S as Fallible>::Error> {
        Ok(RcResolver::from_pos(
            serializer.serialize_interned(field.borrow())?,
        ))
    }
}

impl<T, D, B> DeserializeWith<ArchivedRc<B::Archived, InternFlavor>, T, D> for BorrowIntern<B>
where
    T: Borrow<B> + From<Box<B>>,
    D: Fallible + ?Sized,
    D::Error: Source,
    B: ArchiveUnsized + LayoutRaw + ?Sized,
    B::Archived: DeserializeUnsized<B, D>,
{
    fn deserialize_with(
        field: &ArchivedRc<B::Archived, InternFlavor>,
        deserializer: &mut D,
    ) -> Result<T, <D as Fallible>::Error> {
        let metadata = field.get().deserialize_metadata();
        let layout = B::layout_raw(metadata).into_error()?;
        let data_address = if layout.size() > 0 {
            unsafe { alloc::alloc::alloc(layout) }
        } else {
            polyfill::dangling(&layout).as_ptr()
        };

        let out = rkyv::ptr_meta::from_raw_parts_mut(data_address.cast(), metadata);

        unsafe {
            field.get().deserialize_unsized(deserializer, out)?;
        }
        unsafe { Ok(T::from(Box::from_raw(out))) }
    }
}

/// A basic adapter that can add interning capabilities to a serializer.
///
/// While this struct is useful for ergonomics, it's best to define a custom
/// serializer when combining capabilities across many crates.
#[derive(Debug, Default)]
pub struct InterningAdapter<S, I> {
    serializer: S,
    interning: I,
}

impl<S, I> InterningAdapter<S, I> {
    /// Constructs a new interning adapter from a serializer and interning.
    pub fn new(serializer: S, interning: I) -> Self {
        Self {
            serializer,
            interning,
        }
    }

    /// Consumes the adapter and returns the components.
    #[allow(dead_code)]
    pub fn into_components(self) -> (S, I) {
        (self.serializer, self.interning)
    }

    /// Consumes the adapter and returns the underlying serializer.
    pub fn into_serializer(self) -> S {
        self.serializer
    }
}

unsafe impl<S: Allocator<E>, I, E> Allocator<E> for InterningAdapter<S, I> {
    unsafe fn push_alloc(&mut self, layout: Layout) -> Result<NonNull<[u8]>, E> {
        unsafe { self.serializer.push_alloc(layout) }
    }

    unsafe fn pop_alloc(&mut self, ptr: NonNull<u8>, layout: Layout) -> Result<(), E> {
        unsafe { self.serializer.pop_alloc(ptr, layout) }
    }
}

impl<S: Positional, I> Positional for InterningAdapter<S, I> {
    fn pos(&self) -> usize {
        self.serializer.pos()
    }
}

impl<S: Writer<E>, I, E> Writer<E> for InterningAdapter<S, I> {
    fn write(&mut self, bytes: &[u8]) -> Result<(), E> {
        self.serializer.write(bytes)
    }
}

impl<S: Sharing<E>, I, E> Sharing<E> for InterningAdapter<S, I> {
    fn start_sharing(&mut self, address: usize) -> SharingState {
        self.serializer.start_sharing(address)
    }

    fn finish_sharing(&mut self, address: usize, pos: usize) -> Result<(), E> {
        self.serializer.finish_sharing(address, pos)
    }
}

impl<S, I, T, E> Interning<T, E> for InterningAdapter<S, I>
where
    I: Interning<T, E>,
    T: ?Sized,
{
    fn start_interning(&mut self, value: &T) -> SharingState {
        self.interning.start_interning(value)
    }

    fn finish_interning(&mut self, value: &T, pos: usize) -> Result<(), E> {
        self.interning.finish_interning(value, pos)
    }
}

impl<S, T, E> Interning<T, E> for Strategy<S, E>
where
    S: Interning<T, E> + ?Sized,
    T: ?Sized,
{
    fn start_interning(&mut self, value: &T) -> SharingState {
        S::start_interning(self, value)
    }

    fn finish_interning(&mut self, value: &T, pos: usize) -> Result<(), E> {
        S::finish_interning(self, value, pos)
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::{
        string::{String, ToString},
        vec::Vec,
    };
    use rkyv::{
        access_unchecked,
        api::serialize_using,
        deserialize,
        rancor::{Panic, ResultExt, Strategy},
        ser::{allocator::ArenaHandle, Serializer},
        util::{with_arena, AlignedVec},
        Archive, Archived, Deserialize, Serialize,
    };

    use super::{BorrowIntern, DerefIntern, Intern, Interner, InterningAdapter};

    const USERS: [&str; 4] = [
        "Alice, the leader and brains behind the team",
        "Bob, bodybuilder and the muscle of the operation",
        "Carol, safe-cracker and swindler extraordinaire",
        "Dave, Jumanji master of the spirit dimension",
    ];

    type InterningSerializer<'a, E> = Strategy<
        InterningAdapter<Serializer<AlignedVec<8>, ArenaHandle<'a>, ()>, Interner<String>>,
        E,
    >;

    fn serialize_interned<T, E>(value: &T) -> Result<AlignedVec<8>, E>
    where
        T: for<'a> Serialize<InterningSerializer<'a, E>>,
    {
        with_arena(|arena| {
            let mut serializer = InterningAdapter::new(
                Serializer::new(AlignedVec::<8>::new(), arena.acquire(), ()),
                Interner::default(),
            );

            serialize_using::<_, E>(value, &mut serializer)?;

            Ok(serializer.into_serializer().into_writer())
        })
    }

    #[test]
    fn intern_strings() {
        #[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
        struct Log {
            #[rkyv(with = Intern)]
            user: String,
            code: u16,
        }

        let mut value = Vec::new();
        for i in 0..1000 {
            value.push(Log {
                user: USERS[i % USERS.len()].to_string(),
                code: (i % u16::MAX as usize) as u16,
            });
        }

        let bytes = serialize_interned::<_, Panic>(&value).always_ok();
        assert!(bytes.len() < 20_000);

        let archived = unsafe { access_unchecked::<Archived<Vec<Log>>>(&bytes) };
        for (a, b) in archived.iter().zip(value.iter()) {
            assert_eq!(*a.user, b.user);
            assert_eq!(a.code, b.code);
        }

        let deserialized = deserialize::<Vec<Log>, Panic>(archived).always_ok();
        assert_eq!(deserialized, value);
    }

    #[test]
    fn deref_intern_strings() {
        #[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
        struct Log {
            #[rkyv(with = DerefIntern)]
            user: String,
            code: u16,
        }

        let mut value = Vec::new();
        for i in 0..1000 {
            value.push(Log {
                user: USERS[i % USERS.len()].to_string(),
                code: (i % u16::MAX as usize) as u16,
            });
        }

        let bytes = serialize_interned::<_, Panic>(&value).always_ok();
        assert!(bytes.len() < 20_000);

        let archived = unsafe { access_unchecked::<Archived<Vec<Log>>>(&bytes) };
        for (a, b) in archived.iter().zip(value.iter()) {
            assert_eq!(*a.user, b.user);
            assert_eq!(a.code, b.code);
        }

        let deserialized = deserialize::<Vec<Log>, Panic>(archived).always_ok();
        assert_eq!(deserialized, value);
    }

    #[test]
    fn borrow_intern_strings() {
        #[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
        struct Log {
            #[rkyv(with = BorrowIntern<str>)]
            user: String,
            code: u16,
        }

        let mut value = Vec::new();
        for i in 0..1000 {
            value.push(Log {
                user: USERS[i % USERS.len()].to_string(),
                code: (i % u16::MAX as usize) as u16,
            });
        }

        let bytes = serialize_interned::<_, Panic>(&value).always_ok();
        assert!(bytes.len() < 20_000);

        let archived = unsafe { access_unchecked::<Archived<Vec<Log>>>(&bytes) };
        for (a, b) in archived.iter().zip(value.iter()) {
            assert_eq!(*a.user, b.user);
            assert_eq!(a.code, b.code);
        }

        let deserialized = deserialize::<Vec<Log>, Panic>(archived).always_ok();
        assert_eq!(deserialized, value);
    }
}
