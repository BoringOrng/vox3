use std::ops::{Deref, DerefMut};

use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect, Serialize, Deserialize)]
pub struct Index(u16);

impl Index {
    pub(crate) const EMPTY: Self = Self(u16::MAX);

    pub(crate) const fn new(inner: u16) -> Self {
        Self(inner)
    }

    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == Self::EMPTY.0
    }
}

impl Deref for Index {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Index {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
