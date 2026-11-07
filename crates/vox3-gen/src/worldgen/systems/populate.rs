use bevy::prelude::*;
use vox3_world::position::components::ChunkPos;

use crate::{
    streaming::components::NeedsWorldgen,
    worldgen::{
        components::Generated,
        resources::{
            WorldgenPasses,
            gen_state::{ChunkSlot, GenState},
        },
    },
};

#[expect(clippy::needless_pass_by_value)]
pub fn populate_gen_state(
    mut commands: Commands,
    passes: Res<WorldgenPasses>,
    mut gen_state: ResMut<GenState>,
    chunks_in_flight: Query<(Entity, &ChunkPos), With<NeedsWorldgen>>,
) {
    for (chunk_entity, &chunk_pos) in chunks_in_flight {
        let mut entity_commands = commands.entity(chunk_entity);
        entity_commands.remove::<NeedsWorldgen>();

        if passes.is_empty() {
            entity_commands.insert(Generated);
            continue;
        }

        gen_state
            .slots
            .insert(chunk_pos, ChunkSlot::new(chunk_entity));
    }
}
