use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_egui::EguiPlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::BLACK))
        // .insert_resource(AssetMetaCheck::Never) // might need this wasm hosting (itch.io returns 403s and loader panics)
        .add_plugins(DefaultPlugins);

    #[cfg(feature = "debug")]
    app.add_plugins((EguiPlugin, utils::fps::ScreenDiagsTextPlugin));

    app.run();
}
