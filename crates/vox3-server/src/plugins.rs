use bevy::{app::PluginGroupBuilder, prelude::*, scene::ScenePlugin, state::app::StatesPlugin};

pub struct ServerPlugins;

impl PluginGroup for ServerPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(MinimalPlugins)
            .add(StatesPlugin)
            .add(AssetPlugin::default())
            .add(ScenePlugin)
            .add_group(vox3_world::WorldPlugins)
            .add_group(vox3_sim::SimPlugins)
            .add(crate::networking::NetworkingPlugin)
    }
}
