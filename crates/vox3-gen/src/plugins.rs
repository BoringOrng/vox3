use bevy::app::plugin_group;

plugin_group! {
    pub struct GenPlugins {
        crate::streaming:::StreamingPlugin,
        crate::worldgen:::WorldgenPlugin,
    }
}
