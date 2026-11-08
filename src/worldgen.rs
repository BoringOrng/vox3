use bevy::prelude::*;
use vox3_gen::worldgen::traits::{
    Feature, WorldgenAppExt,
    gen_pass::{GenContext, GenPass},
};
use vox3_world::{
    chunk::components::ChunkData, coords::ChunkLocalPos, position::components::ChunkPos,
};

pub struct WorldgenPlugin;

impl Plugin for WorldgenPlugin {
    fn build(&self, app: &mut App) {
        app.worldgen_pass(FlatWorldgen).worldgen_pass(PlusPass);
    }
}

struct FlatWorldgen;

impl GenPass for FlatWorldgen {
    fn reach(&self) -> u32 {
        0
    }

    fn run(&self, context: GenContext) -> ChunkData {
        let mut chunk_data = ChunkData::new();

        if context.pos.y() <= 0 {
            chunk_data.fill(ChunkLocalPos::MIN..ChunkLocalPos::MAX, true);
            chunk_data.insert(ChunkLocalPos::MAX, true);
        }

        chunk_data
    }
}

struct PlusPass;

impl GenPass for PlusPass {
    fn reach(&self) -> u32 {
        1
    }

    fn run(&self, context: GenContext) -> ChunkData {
        let mut next_data = context
            .previous
            .as_ref()
            .map_or_default(|previous| (**previous).clone());

        let tree_feature = TreeFeature;
        tree_feature.decorate(&mut next_data, context.pos, &context.neighbors);

        next_data
    }
}

struct TreeFeature;

impl Feature for TreeFeature {
    fn get_origins(&self, pos: ChunkPos) -> Vec<ChunkLocalPos> {
        if pos.y() != 1 {
            return Vec::new();
        }

        vec![ChunkLocalPos::new(0, 0, 0)]
    }

    fn is_support_block(&self, block: Option<bool>) -> bool {
        block.is_some_and(|id| id)
    }

    fn fill_pattern(&self) -> Vec<(IVec3, bool)> {
        let trunk_height = 5;
        let crown_height = 7;
        let crown_depth = 3;

        let mut blocks = Vec::new();

        for y in 0..trunk_height {
            blocks.push((IVec3::new(0, y, 0), true));
        }

        for y in trunk_height..crown_height {
            for x in -1..=1 {
                for z in -1..=1 {
                    blocks.push((IVec3::new(x, y, z), true));
                }
            }
        }

        for y in crown_depth..trunk_height {
            for x in -2..=2 {
                for z in -2..=2 {
                    blocks.push((IVec3::new(x, y, z), true));
                }
            }
        }

        blocks
    }
}
