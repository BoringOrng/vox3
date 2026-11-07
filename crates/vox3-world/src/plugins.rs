use bevy::app::plugin_group;

plugin_group! {
    pub struct WorldPlugins {
        crate::chunk::plugin:::ChunkPlugin,
        crate::position::plugin:::PositionPlugin,
    }
}
