mod material;
mod meshing;
mod networking;
mod player;
mod position;

// temporary
mod worldgen;

use bevy::{app::PluginGroupBuilder, prelude::*};

#[cfg(feature = "dev")]
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

#[derive(Default)]
struct ClientPlugins;

impl PluginGroup for ClientPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();

        builder = builder
            .add_group(DefaultPlugins)
            .add_group(vox3_world::WorldPlugins)
            .add_group(vox3_sim::SimPlugins)
            .add_group(vox3_gen::GenPlugins)
            .add(position::PositionPlugin)
            .add(player::PlayerPlugin)
            .add(networking::NetworkingPlugin)
            .add(material::MaterialPlugin)
            .add(meshing::MeshingPlugin)
            .add(worldgen::WorldgenPlugin);

        #[cfg(feature = "dev")]
        {
            builder = builder
                .add(EguiPlugin::default())
                .add(WorldInspectorPlugin::new());
        }

        builder
    }
}

fn main() -> AppExit {
    App::new().add_plugins(ClientPlugins).run()
}
