use bevy::prelude::*;
use bevy_replicon::shared::backend::connected_client::ConnectedClient;

#[expect(clippy::needless_pass_by_value)]
pub fn on_connected_client(trigger: On<Add<ConnectedClient>>) {
    info!("Client connected! {}", trigger.entity);
}
