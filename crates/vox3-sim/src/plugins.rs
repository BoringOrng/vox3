use bevy::app::plugin_group;

plugin_group! {
    pub struct SimPlugins {
        crate::ticking:::TickingPlugin,
    }
}
