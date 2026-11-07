// temporary
mod worldgen;

use bevy::prelude::*;

#[cfg(feature = "dev")]
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        vox3_client::ClientPlugins,
        worldgen::WorldgenPlugin,
    ));

    #[cfg(feature = "dev")]
    {
        app.add_plugins((EguiPlugin::default(), WorldInspectorPlugin::new()));
    }

    app.run()
}
