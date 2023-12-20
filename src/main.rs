use std::ops::Mul;

use bevy::{
  core_pipeline::{
    bloom::BloomSettings,
    experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin},
    tonemapping::Tonemapping,
    Skybox,
  },
  pbr::{CascadeShadowConfigBuilder, ScreenSpaceAmbientOcclusionBundle},
  prelude::*,
  render::camera::ScalingMode,
};

#[cfg(feature = "debug")]
use bevy_egui::EguiPlugin;
use simulation::SimulationPlugin;
use utils::lerp;

mod camera;
mod player;
mod scene;

fn main() {
  let mut app = App::new();
  app
    .insert_resource(ClearColor(Color::BLACK))
    // .insert_resource(AssetMetaCheck::Never) // might need this wasm hosting (itch.io returns 403s and loader panics)
    .add_plugins((DefaultPlugins, TemporalAntiAliasPlugin, SimulationPlugin))
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
        player::play_animations,
        player::update_player,
        camera::update_camera,
      ),
    );

  #[cfg(feature = "debug")]
  app.add_plugins((EguiPlugin, utils::fps::ScreenDiagsTextPlugin));

  app.run();
}
