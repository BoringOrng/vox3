use bevy::prelude::*;

use crate::position::sets::PositionSystems;

#[derive(Default)]
pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PostUpdate,
            (
                PositionSystems::Origin,
                PositionSystems::Sync,
                PositionSystems::PostPropagate,
            )
                .chain(),
        )
        .configure_sets(
            PostUpdate,
            (
                PositionSystems::Sync.before(TransformSystems::Propagate),
                PositionSystems::PostPropagate.after(TransformSystems::Propagate),
            ),
        );
    }
}
