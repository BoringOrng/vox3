use bevy::prelude::*;

use crate::{
    streaming::sets::StreamingSystems,
    worldgen::{
        resources::{WorldgenPasses, gen_state::GenState},
        sets::WorldgenSystems,
        systems,
    },
};

#[derive(Default)]
pub struct WorldgenPlugin;

impl Plugin for WorldgenPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            FixedUpdate,
            (
                WorldgenSystems::Populate.after(StreamingSystems::Process),
                WorldgenSystems::Generate,
            )
                .chain(),
        )
        .init_resource::<GenState>()
        .init_resource::<WorldgenPasses>()
        .add_systems(
            FixedUpdate,
            (
                systems::populate::populate_gen_state.in_set(WorldgenSystems::Populate),
                (
                    systems::generate::spawn_generate_tasks,
                    systems::generate::poll_generation_tasks,
                )
                    .in_set(WorldgenSystems::Generate),
            ),
        );
    }
}
