use std::sync::Arc;

use bevy::prelude::*;
use vox3_gen::worldgen::traits::{GenPass, WorldgenAppExt};
use vox3_world::{
    chunk::components::ChunkData, coords::ChunkLocalPos, position::components::ChunkPos,
};

pub struct WorldgenPlugin;

impl Plugin for WorldgenPlugin {
    fn build(&self, app: &mut App) {
        app.worldgen_pass(FlatWorldgen);
    }
}

struct FlatWorldgen;

impl GenPass for FlatWorldgen {
    fn reach(&self) -> u32 {
        0
    }

    fn run(&self, pos: ChunkPos, _chunk_data: Option<Arc<ChunkData>>) -> ChunkData {
        let mut chunk_data = ChunkData::new();

        if pos.y() <= 0 {
            chunk_data.fill(ChunkLocalPos::MIN..ChunkLocalPos::MAX, true);
            chunk_data.insert(ChunkLocalPos::MAX, true);
        }

        chunk_data
    }
}
