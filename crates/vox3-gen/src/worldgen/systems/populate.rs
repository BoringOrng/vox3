use bevy::prelude::*;
use vox3_world::position::components::ChunkPos;

use crate::{
    streaming::components::NeedsWorldgen,
    worldgen::resources::gen_state::{ChunkSlot, GenState},
};

pub fn populate_gen_state(
    mut commands: Commands,
    mut gen_state: ResMut<GenState>,
    chunks_in_flight: Query<(Entity, &ChunkPos), With<NeedsWorldgen>>,
) {
    for (chunk_entity, &chunk_pos) in chunks_in_flight {
        gen_state
            .slots
            .insert(chunk_pos, ChunkSlot::new(chunk_entity));

        commands.entity(chunk_entity).remove::<NeedsWorldgen>();
    }
}
