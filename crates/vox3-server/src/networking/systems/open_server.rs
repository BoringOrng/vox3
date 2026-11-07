use aeronet_replicon::server::AeronetRepliconServer;
use aeronet_websocket::server::{ServerConfig, WebSocketServer};
use bevy::prelude::*;

pub fn open_server(mut commands: Commands) {
    let config = ServerConfig::builder()
        .with_bind_default(25570)
        .with_no_encryption();

    commands
        .spawn_scene(bsn! {
            #Server
            AeronetRepliconServer
        })
        .queue(WebSocketServer::open(config));
}
