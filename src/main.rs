use animation::AnimationController;
use assets::RonAssetApp;
use bevy::{
  core_pipeline::experimental::taa::TemporalAntiAliasPlugin,
  input::common_conditions::input_toggle_active, prelude::*,
};

#[cfg(feature = "debug")]
use bevy_egui::EguiPlugin;
#[cfg(feature = "debug")]
use bevy_inspector_egui::DefaultInspectorConfigPlugin;

use bevy_scene_hook::HookPlugin;
use simulation::SimulationPlugin;

mod animation;
mod camera;
#[cfg(feature = "debug")]
mod debug;
mod player;
mod scene;

fn main() {
  let mut app = App::new();
  app
    .insert_resource(ClearColor(Color::BLACK))
    // .insert_resource(AssetMetaCheck::Never) // might need this wasm hosting (itch.io returns 403s
    // and loader panics)
    .add_plugins((
      DefaultPlugins,
      TemporalAntiAliasPlugin,
      SimulationPlugin,
      HookPlugin,
    ))
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
        (animation::find_rig_target, animation::play_animations).chain(),
        player::update_player,
        camera::update_camera,
      ),
    );

  #[cfg(feature = "debug")]
  app
    .add_plugins((
      EguiPlugin,
      DefaultInspectorConfigPlugin,
      utils::fps::ScreenDiagsTextPlugin,
    ))
    .add_systems(
      Update,
      debug::inspector_ui.run_if(input_toggle_active(true, KeyCode::Escape)),
    );

  app.run();
}
