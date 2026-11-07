use std::sync::Arc;

use vox3_world::{chunk::components::ChunkData, position::components::ChunkPos};

pub trait GenPass: Send + Sync + 'static {
    fn reach(&self) -> u32;
    fn run(&self, context: GenContext) -> ChunkData;
}

#[derive(Clone, Debug)]
pub struct GenContext {
    pub pos: ChunkPos,
    pub previous: Option<Arc<ChunkData>>,
    pub neighbors: Vec<NeighborChunkData>,
}

#[derive(Clone, Debug)]
pub struct NeighborChunkData {
    pub pos: ChunkPos,
    pub data: Arc<ChunkData>,
}
