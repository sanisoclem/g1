use bevy::prelude::*;
use simulation::worldgen::{WorldCommand, WorldSeed};

pub fn setup_test_scene(
  mut cmd: Commands,
  mut worldgen_cmd: EventWriter<WorldCommand>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut meshes: ResMut<Assets<Mesh>>,
) {
  worldgen_cmd.send(WorldCommand::CreateWorld {
    world_blueprint: "default.world.ron".to_owned(),
    seed: WorldSeed::default(),
  });
  cmd.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 9000.0,
      range: 100.,
      shadows_enabled: true,
      ..default()
    },
    transform: Transform::from_xyz(8.0, 16.0, 8.0),
    ..default()
  });
  // light
  cmd.spawn(DirectionalLightBundle {
    directional_light: DirectionalLight {
      color: Color::rgb(0.98, 0.95, 0.82),
      shadows_enabled: true,
      ..default()
    },
    transform: Transform::from_xyz(0.0, 0.0, 0.0)
      .looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
    ..default()
  });
}
