extern crate alloc;

use alloc::borrow::ToOwned;
use core::{borrow::Borrow, error::Error, fmt, hash::Hash};
use std::collections::HashMap;

use rkyv::{
    rancor::{fail, Source},
    ser::sharing::SharingState,
};

use super::Interning;

/// A general-purpose value interner.
pub struct Interner<T> {
    value_to_pos: HashMap<T, Option<usize>>,
}

impl<T> Interner<T> {
    /// Returns a new, empty interner.
    pub fn new() -> Self {
        Self {
            value_to_pos: HashMap::new(),
        }
    }
}

impl<T> Default for Interner<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct NotStarted;

impl fmt::Display for NotStarted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "value was not started interning")
    }
}

impl Error for NotStarted {}

#[derive(Debug)]
struct AlreadyFinished;

impl fmt::Display for AlreadyFinished {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "value was already finished interning")
    }
}

impl Error for AlreadyFinished {}

impl<T, E> Interning<T, E> for Interner<T::Owned>
where
    T::Owned: Hash + Eq + Borrow<T>,
    T: Hash + Eq + ToOwned + ?Sized,
    E: Source,
{
    fn start_interning(&mut self, value: &T) -> SharingState {
        match self.value_to_pos.get(value) {
            None => {
                self.value_to_pos.insert(value.to_owned(), None);
                SharingState::Started
            }
            Some(None) => SharingState::Pending,
            Some(Some(pos)) => SharingState::Finished(*pos),
        }
    }

    fn finish_interning(&mut self, value: &T, pos: usize) -> Result<(), E> {
        match self.value_to_pos.get_mut(value) {
            None => fail!(NotStarted),
            Some(Some(_)) => fail!(AlreadyFinished),
            Some(x) => {
                *x = Some(pos);
                Ok(())
            }
        }
    }
}
