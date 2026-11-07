use std::ops::Range;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{coords::ChunkLocalPos, storage::PVec};

#[derive(Component, Reflect, Default, Debug, Serialize, Deserialize)]
pub struct ChunkData(PVec<bool>);

impl ChunkData {
    #[must_use]
    pub fn new() -> Self {
        Self(PVec::new())
    }

    #[must_use]
    pub fn get(&self, p: ChunkLocalPos) -> Option<bool> {
        self.0.get(p.index())
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn insert(&mut self, p: ChunkLocalPos, v: bool) -> Option<bool> {
        self.0.set(p.index(), v)
    }

    pub fn fill(&mut self, r: Range<ChunkLocalPos>, v: bool) {
        self.0.fill(r.start.index()..r.end.index(), v);
    }

    pub fn remove(&mut self, p: ChunkLocalPos) -> Option<bool> {
        self.0.remove(p.index())
    }

    pub fn iter(&self) -> impl Iterator<Item = (ChunkLocalPos, bool)> {
        self.0
            .iter()
            .map(|(i, v)| (ChunkLocalPos::from_index(i), v))
    }
}
