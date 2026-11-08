use bevy::math::IVec3;
use vox3_world::{
    chunk::components::{Chunk, ChunkData},
    coords::ChunkLocalPos,
    position::components::ChunkPos,
};

use crate::worldgen::traits::gen_pass::NeighborChunkData;

pub trait Feature {
    fn get_origins(&self, pos: ChunkPos) -> Vec<ChunkLocalPos>;

    fn is_support_block(&self, block: Option<bool>) -> bool;

    fn fill_pattern(&self) -> Vec<(IVec3, bool)>;

    fn decorate(
        &self,
        chunk_data: &mut ChunkData,
        center_chunk: ChunkPos,
        neighbors: &[NeighborChunkData],
    ) {
        let mut origin_chunks = vec![center_chunk];
        origin_chunks.extend(neighbors.iter().map(|n| n.pos));

        for origin_chunk in origin_chunks {
            for origin in self.get_origins(origin_chunk) {
                if !self.has_support(neighbors, origin_chunk, origin) {
                    continue;
                }

                self.insert(chunk_data, center_chunk, origin_chunk, origin);
            }
        }
    }

    fn has_support(
        &self,
        neighbors: &[NeighborChunkData],
        origin_chunk: ChunkPos,
        origin: ChunkLocalPos,
    ) -> bool {
        let support_chunk = origin_chunk + ChunkPos::new(0, -1, 0);

        neighbors
            .iter()
            .find(|neighbor| neighbor.pos == support_chunk)
            .is_some_and(|neighbor| {
                self.is_support_block(neighbor.data.get(ChunkLocalPos::new(
                    origin.x(),
                    Chunk::SIZE - 1,
                    origin.z(),
                )))
            })
    }

    fn insert(
        &self,
        chunk_data: &mut ChunkData,
        target_chunk: ChunkPos,
        origin_chunk: ChunkPos,
        origin: ChunkLocalPos,
    ) {
        let local_origin = origin.as_ivec3();
        let size = IVec3::splat(Chunk::SIZE.into());

        for (offset, block) in self.fill_pattern() {
            let cell = local_origin + offset;

            let chunk_offset = cell.div_euclid(size);
            let cell_chunk = origin_chunk + ChunkPos::from_ivec3(chunk_offset);
            if cell_chunk != target_chunk {
                continue;
            }

            // TODO: nuke this once actual block types exist
            if block {
                let [x, y, z] = cell.rem_euclid(size).as_u8vec3().to_array();
                chunk_data.insert(ChunkLocalPos::new(x, y, z), block);
            }
        }
    }
}
