use std::sync::Arc;

use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, block_on, poll_once},
};
use vox3_world::position::components::ChunkPos;

use crate::worldgen::{
    components::Generated,
    resources::{WorldgenPasses, gen_state::GenState},
    traits::gen_pass::{GenContext, NeighborChunkData},
};

#[expect(clippy::needless_pass_by_value)]
pub fn spawn_generate_tasks(passes: Res<WorldgenPasses>, mut gen_state: ResMut<GenState>) {
    if passes.is_empty() || gen_state.slots.is_empty() {
        return;
    }

    let mut jobs = Vec::new();

    for (&pos, slot) in &gen_state.slots {
        if slot.complete || slot.running.is_some() {
            continue;
        }

        let stage = slot.history.len();
        let Some(pass) = passes.get(stage) else {
            continue;
        };

        let Some(neighbors) = neighbors_for_stage(pos, stage, pass.reach(), &gen_state) else {
            continue;
        };

        jobs.push((
            pos,
            Arc::clone(pass),
            GenContext {
                pos,
                previous: slot.history.last().cloned(),
                neighbors,
            },
        ));
    }

    let pool = AsyncComputeTaskPool::get();
    for (pos, pass, context) in jobs {
        if let Some(slot) = gen_state.slots.get_mut(&pos) {
            slot.running = Some(pool.spawn(async move { pass.run(context) }));
        }
    }
}

#[expect(clippy::needless_pass_by_value)]
pub fn poll_generation_tasks(
    mut commands: Commands,
    passes: Res<WorldgenPasses>,
    mut gen_state: ResMut<GenState>,
) {
    if passes.is_empty() || gen_state.slots.is_empty() {
        return;
    }

    for slot in gen_state.slots.values_mut() {
        let Some(task) = slot.running.as_mut() else {
            continue;
        };

        let Some(chunk_data) = block_on(poll_once(task)) else {
            continue;
        };

        slot.running = None;

        if slot.history.len() + 1 == passes.len() {
            slot.complete = true;

            commands
                .entity(slot.chunk_entity)
                .insert((chunk_data, Generated));
        } else {
            slot.history.push(Arc::new(chunk_data));
        }
    }
}

fn neighbors_for_stage(
    pos: ChunkPos,
    stage: usize,
    reach: u32,
    gen_state: &GenState,
) -> Option<Vec<NeighborChunkData>> {
    if stage == 0 || reach == 0 {
        return Some(Vec::new());
    }

    let radius = i32::try_from(reach).ok()?;
    let mut neighbors = Vec::new();

    for x in -radius..=radius {
        for y in -radius..=radius {
            for z in -radius..=radius {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }

                let neighbor_pos = pos + ChunkPos::from_ivec3(IVec3::new(x, y, z));
                let slot = gen_state.slots.get(&neighbor_pos)?;
                let previous = slot.history.get(stage - 1)?.clone();
                neighbors.push(NeighborChunkData {
                    pos: neighbor_pos,
                    data: previous,
                });
            }
        }
    }

    Some(neighbors)
}
