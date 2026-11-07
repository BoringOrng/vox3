use aeronet_replicon::server::AeronetRepliconServerPlugin;
use aeronet_websocket::server::WebSocketServerPlugin;
use bevy::prelude::*;
use bevy_replicon::RepliconPlugins;

use crate::networking::{observers, systems};

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RepliconPlugins,
            WebSocketServerPlugin,
            AeronetRepliconServerPlugin,
        ))
        .add_systems(Startup, systems::open_server)
        .add_observer(observers::on_connected_client);
    }
}
