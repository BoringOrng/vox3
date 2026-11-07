use bevy::prelude::*;

use crate::streaming::{
    resources::{StreamingConfig, stream_queue::StreamQueue},
    sets::StreamingSystems,
    systems,
};

#[derive(Default)]
pub struct StreamingPlugin;

impl Plugin for StreamingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StreamQueue>()
            .init_resource::<StreamingConfig>()
            .configure_sets(
                FixedUpdate,
                (StreamingSystems::Discover, StreamingSystems::Process).chain(),
            )
            .add_systems(
                FixedUpdate,
                (
                    systems::discover::discover_chunks.in_set(StreamingSystems::Discover),
                    systems::process::process_spawn_queue.in_set(StreamingSystems::Process),
                ),
            );
    }
}
