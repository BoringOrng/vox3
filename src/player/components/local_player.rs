use bevy::prelude::*;
use vox3_gen::streaming::components::StreamingAnchor;
use vox3_sim::player::components::Player;
use vox3_world::position::components::{ChunkPos, VoxelPos};

use crate::position::components::FloatingOrigin;

#[derive(Component, Reflect, Default, Clone, Copy)]
#[reflect(Component)]
#[require(
    Player,
    Name,
    Camera3d,
    FloatingOrigin,
    StreamingAnchor,
    VoxelPos {
        chunk: ChunkPos::new(0, 1, 0),
        local: Vec3::new(16.5, 2.0, 16.5)
    }
)]
pub struct LocalPlayer;
