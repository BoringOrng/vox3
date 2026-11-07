use bevy::prelude::*;

use crate::material::resources::VoxelMaterial;

pub struct MaterialPlugin;

impl Plugin for MaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::pbr::MaterialPlugin::<VoxelMaterial>::default())
            .init_resource::<VoxelMaterial>();
    }
}
