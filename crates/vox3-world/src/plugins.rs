use bevy::app::plugin_group;

plugin_group! {
    #[derive(Default)]
    pub struct WorldPlugins {
        crate::chunk::plugin:::ChunkPlugin,
        crate::position::plugin:::PositionPlugin,
    }
}
