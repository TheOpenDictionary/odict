use super::{Interner, InterningAdapter};
use rkyv::{
    api::serialize_using,
    rancor::Strategy,
    ser::{allocator::ArenaHandle, Serializer},
    util::{with_arena, AlignedVec},
    Serialize,
};

type InterningSerializer<'a, E> =
    Strategy<InterningAdapter<Serializer<AlignedVec<8>, ArenaHandle<'a>, ()>, Interner<String>>, E>;

pub fn serialize_interned<T, E>(value: &T) -> Result<AlignedVec<8>, E>
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
