use std::sync::Arc;

use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, block_on, poll_once},
};

use crate::worldgen::{
    components::Generated,
    resources::{WorldgenPasses, gen_state::GenState},
};

#[expect(clippy::needless_pass_by_value)]
pub fn spawn_generate_tasks(passes: Res<WorldgenPasses>, mut gen_state: ResMut<GenState>) {
    if passes.is_empty() || gen_state.slots.is_empty() {
        return;
    }

    let pool = AsyncComputeTaskPool::get();

    for (&pos, slot) in &mut gen_state.slots {
        if slot.running.is_some() {
            continue;
        }

        let stage = slot.history.len();
        let Some(pass) = passes.get(stage) else {
            continue;
        };

        let pass = Arc::clone(pass);
        let prev = slot.history.last().cloned();

        slot.running = Some(pool.spawn(async move { pass.run(pos, prev) }));
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

    let mut finished = Vec::new();

    for (&pos, slot) in &mut gen_state.slots {
        let Some(task) = slot.running.as_mut() else {
            continue;
        };

        let Some(chunk_data) = block_on(poll_once(task)) else {
            continue;
        };

        slot.running = None;

        if slot.history.len() + 1 == passes.len() {
            commands
                .entity(slot.chunk_entity)
                .insert((chunk_data, Generated));

            finished.push(pos);
        } else {
            slot.history.push(Arc::new(chunk_data));
        }
    }

    for pos in finished {
        gen_state.slots.remove(&pos);
    }
}
