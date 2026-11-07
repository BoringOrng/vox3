use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Reflect, Serialize, Deserialize)]
pub struct Slot<T: Copy> {
    item: T,
    refs: u16,
}

impl<T: Copy> Slot<T> {
    #[must_use]
    pub(crate) const fn new(item: T) -> Self {
        Self { item, refs: 1 }
    }

    #[must_use]
    pub const fn item(self) -> T {
        self.item
    }

    #[must_use]
    pub const fn refs(self) -> u16 {
        self.refs
    }

    #[must_use]
    pub(crate) const fn refs_mut(&mut self) -> &mut u16 {
        &mut self.refs
    }
}
