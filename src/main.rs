use animation::AnimationController;
use assets::RonAssetApp;
use bevy::{core_pipeline::experimental::taa::TemporalAntiAliasPlugin, prelude::*};

#[cfg(feature = "debug")]
use bevy_egui::EguiPlugin;

use simulation::SimulationPlugin;

mod animation;
mod camera;
mod player;
mod scene;

fn main() {
  let mut app = App::new();
  app
    .insert_resource(ClearColor(Color::BLACK))
    // .insert_resource(AssetMetaCheck::Never) // might need this wasm hosting (itch.io returns 403s
    // and loader panics)
    .add_plugins((DefaultPlugins, TemporalAntiAliasPlugin, SimulationPlugin))
    .register_ron_asset::<AnimationController>()
    .add_systems(
      Startup,
      (
        scene::setup_test_scene,
        player::setup_player,
        camera::setup_camera,
      ),
    )
    .add_systems(
      Update,
      (
        animation::play_animations,
        player::update_player,
        camera::update_camera,
      ),
    );

  #[cfg(feature = "debug")]
  app.add_plugins((EguiPlugin, utils::fps::ScreenDiagsTextPlugin));

  app.run();
}
