use bevy::prelude::*;
use vox3_world::chunk::resources::ChunkMap;

use crate::worldgen::resources::gen_state::GenState;

#[expect(clippy::needless_pass_by_value)]
pub fn prune_gen_state(mut gen_state: ResMut<GenState>, chunk_map: Res<ChunkMap>) {
    if !chunk_map.is_changed() {
        return;
    }

    gen_state
        .slots
        .retain(|&pos, _| chunk_map.contains_key(pos));
}
