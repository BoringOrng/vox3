use bevy::prelude::*;

use crate::ticking::constants::TICK_SPEED;

#[derive(Default)]
pub struct TickingPlugin;

impl Plugin for TickingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(TICK_SPEED));
    }
}
