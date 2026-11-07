use bevy::prelude::*;

use crate::position::components::ChunkPos;

#[derive(Clone, Copy, Debug, Default, Component)]
#[require(Transform)]
pub struct VoxelPos {
    pub chunk: ChunkPos,
    pub local: Vec3,
}

impl VoxelPos {
    #[must_use]
    pub const fn from_chunk(chunk: ChunkPos) -> Self {
        Self {
            chunk,
            local: Vec3::ZERO,
        }
    }
}
