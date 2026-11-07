use bevy::prelude::*;

use crate::player::systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_player);
    }
}
