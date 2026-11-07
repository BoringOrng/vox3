use bevy::prelude::*;

use super::resources::ChunkMap;

#[derive(Default)]
pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkMap>();
    }
}
