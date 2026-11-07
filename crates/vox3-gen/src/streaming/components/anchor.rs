use bevy::prelude::*;
use vox3_world::position::components::{ChunkPos, VoxelPos};

#[derive(Component, Default)]
pub struct LastPos(pub(in crate::streaming) Option<ChunkPos>);

#[derive(Component, Debug, Default, Clone, Copy)]
#[require(VoxelPos, LastPos)]
pub struct StreamingAnchor;
