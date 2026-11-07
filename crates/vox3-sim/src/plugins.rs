use bevy::app::plugin_group;

plugin_group! {
    #[derive(Default)]
    pub struct SimPlugins {
        crate::ticking:::TickingPlugin,
    }
}
