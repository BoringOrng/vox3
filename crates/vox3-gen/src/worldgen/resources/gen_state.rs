use std::sync::Arc;

use bevy::{platform::collections::HashMap, prelude::*, tasks::Task};
use vox3_world::{chunk::components::ChunkData, position::components::ChunkPos};

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct GenState {
    pub(in crate::worldgen) slots: HashMap<ChunkPos, ChunkSlot>,
}

#[derive(Reflect)]
pub struct ChunkSlot {
    pub(in crate::worldgen) history: Vec<Arc<ChunkData>>,

    pub(in crate::worldgen) chunk_entity: Entity,

    #[reflect(ignore)]
    pub(in crate::worldgen) running: Option<Task<ChunkData>>,
}

impl ChunkSlot {
    #[must_use]
    pub fn new(chunk_entity: Entity) -> Self {
        Self {
            history: default(),
            chunk_entity,
            running: default(),
        }
    }
}
