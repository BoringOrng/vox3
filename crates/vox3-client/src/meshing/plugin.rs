use bevy::prelude::*;
use vox3_world::position::sets::PositionSystems;

use crate::meshing::systems;

#[derive(Default)]
pub struct MeshingPlugin;

impl Plugin for MeshingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<systems::MeshingState>().add_systems(
            PostUpdate,
            systems::update_chunk_meshes.after(PositionSystems::Sync),
        );
    }
}
