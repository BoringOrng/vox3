use aeronet_replicon::client::AeronetRepliconClientPlugin;
use bevy::prelude::*;
use bevy_replicon::RepliconPlugins;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RepliconPlugins, AeronetRepliconClientPlugin));
    }
}
