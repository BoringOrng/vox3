use bevy::app::plugin_group;

pub mod material;
pub mod meshing;
pub mod networking;
pub mod player;
pub mod position;

plugin_group! {
    pub struct ClientPlugins {
        material:::MaterialPlugin,
        meshing:::MeshingPlugin,
        networking:::NetworkingPlugin,
        player:::PlayerPlugin,
        position:::PositionPlugin,

        #[plugin_group]
        vox3_world:::WorldPlugins,

        #[plugin_group]
        vox3_sim:::SimPlugins,

        #[plugin_group]
        vox3_gen:::GenPlugins,
    }
}
