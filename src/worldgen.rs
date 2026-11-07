use bevy::prelude::*;
use vox3_gen::worldgen::traits::{
    WorldgenAppExt,
    gen_pass::{GenContext, GenPass},
};
use vox3_world::{
    chunk::components::{Chunk, ChunkData},
    coords::ChunkLocalPos,
    direction::Cardinal,
    position::components::ChunkPos,
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
        let mut origin_chunks = vec![context.pos];
        origin_chunks.extend(
            context
                .neighbors
                .iter()
                .filter(|neighbor| neighbor.pos.y() == context.pos.y())
                .map(|neighbor| neighbor.pos),
        );

        for origin_chunk in origin_chunks {
            for origin in Self::get_plus_origins(origin_chunk) {
                if !Self::has_support(&context, origin_chunk, origin) {
                    continue;
                }

                Self::insert_plus(&mut next_data, context.pos, origin_chunk, origin);
            }
        }

        next_data
    }
}

impl PlusPass {
    fn get_plus_origins(chunk_pos: ChunkPos) -> Vec<ChunkLocalPos> {
        if chunk_pos.y() != 1 {
            return Vec::new();
        }

        let mut origins = Vec::new();

        for x in 0..Chunk::SIZE {
            for z in 0..Chunk::SIZE {
                if (x + z).is_multiple_of(2) && (x.wrapping_mul(z)).is_multiple_of(95) {
                    origins.push(ChunkLocalPos::new(x, 0, z));
                }
            }
        }

        origins
    }

    fn has_support(context: &GenContext, origin_chunk: ChunkPos, origin: ChunkLocalPos) -> bool {
        let support_chunk = origin_chunk + ChunkPos::new(0, -1, 0);
        context
            .neighbors
            .iter()
            .find(|neighbor| neighbor.pos == support_chunk)
            .is_some_and(|neighbor| {
                neighbor
                    .data
                    .get(ChunkLocalPos::new(origin.x(), Chunk::SIZE - 1, origin.z()))
                    == Some(true)
            })
    }

    fn insert_plus(
        chunk_data: &mut ChunkData,
        target_chunk: ChunkPos,
        origin_chunk: ChunkPos,
        origin: ChunkLocalPos,
    ) {
        let local_origin = IVec3::new(
            i32::from(origin.x()),
            i32::from(origin.y()),
            i32::from(origin.z()),
        );
        let size = i32::from(Chunk::SIZE);
        let offsets = [
            IVec3::ZERO,
            Cardinal::North.to_ivec3(),
            Cardinal::South.to_ivec3(),
            Cardinal::East.to_ivec3(),
            Cardinal::West.to_ivec3(),
        ];

        for offset in offsets {
            let cell = local_origin + offset;
            let chunk_offset = IVec3::new(
                cell.x.div_euclid(size),
                cell.y.div_euclid(size),
                cell.z.div_euclid(size),
            );
            let cell_chunk = origin_chunk + ChunkPos::from_ivec3(chunk_offset);
            if cell_chunk != target_chunk {
                continue;
            }

            let local = IVec3::new(
                cell.x.rem_euclid(size),
                cell.y.rem_euclid(size),
                cell.z.rem_euclid(size),
            );
            let [x, y, z] = local.as_uvec3().as_u8vec3().to_array();
            chunk_data.insert(ChunkLocalPos::new(x, y, z), true);
        }
    }
}
