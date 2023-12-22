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

use bevy_common_assets::ron::RonAssetPlugin;
#[cfg(feature = "debug")]
use bevy_egui::EguiPlugin;
use player::Player;
use simulation::SimulationPlugin;
use utils::lerp;

mod camera;
mod player;
mod scene;
mod creature;
fn main() {
  let mut app = App::new();
  app
    .insert_resource(ClearColor(Color::BLACK))
    // .insert_resource(AssetMetaCheck::Never) // might need this wasm hosting (itch.io returns 403s and loader panics)
    .add_plugins((
      DefaultPlugins,
      TemporalAntiAliasPlugin,
      SimulationPlugin,
      RonAssetPlugin::<player::CreatureBlueprint>::new(&["creature.ron"]),
    ))
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
