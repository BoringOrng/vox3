use bevy::prelude::*;

use crate::player::components::LocalPlayer;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        #Orng
        LocalPlayer
    });
}
