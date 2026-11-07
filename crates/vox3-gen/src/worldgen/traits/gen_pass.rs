use std::sync::Arc;

use vox3_world::{chunk::components::ChunkData, position::components::ChunkPos};

pub trait GenPass: Send + Sync + 'static {
    fn reach(&self) -> u32;
    fn run(&self, pos: ChunkPos, chunk_data: Option<Arc<ChunkData>>) -> ChunkData;
}
