use bevy::prelude::*;
use vox3_world::position::sets::PositionSystems;

use crate::position::{resources::RenderOrigin, systems};

pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RenderOrigin>().add_systems(
            PostUpdate,
            (
                systems::origin::update_origin.in_set(PositionSystems::Origin),
                (
                    systems::sync::sync_transforms,
                    systems::sync::sync_chunk_pos,
                )
                    .in_set(PositionSystems::Sync),
            ),
        );
    }
}
