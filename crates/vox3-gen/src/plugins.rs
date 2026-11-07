use bevy::app::plugin_group;

plugin_group! {
    #[derive(Default)]
    pub struct GenPlugins {
        crate::streaming:::StreamingPlugin,
        crate::worldgen:::WorldgenPlugin,
    }
}
