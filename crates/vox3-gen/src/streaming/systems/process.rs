use bevy::prelude::*;
use vox3_world::chunk::{components::Chunk, resources::ChunkMap};

use crate::streaming::{components::NeedsWorldgen, resources::stream_queue::StreamQueue};

const BATCH_SIZE: usize = 128;

#[expect(clippy::needless_pass_by_value)]
pub fn process_spawn_queue(
    mut commands: Commands,
    mut spawn_queue: ResMut<StreamQueue>,
    chunk_map: Res<ChunkMap>,
) {
    let to_spawn: Vec<_> = spawn_queue
        .pop_batch(BATCH_SIZE, &chunk_map)
        .into_iter()
        .map(|pos| (pos, Chunk, NeedsWorldgen))
        .collect();

    if !to_spawn.is_empty() {
        commands.spawn_batch(to_spawn);
    }
}
