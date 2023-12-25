use bevy::prelude::*;

pub fn setup_test_scene(
  mut cmd: Commands,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut meshes: ResMut<Assets<Mesh>>,
) {
  // plane
  cmd.spawn(PbrBundle {
    mesh: meshes.add(shape::Plane::from_size(500.0).into()),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    ..default()
  })
  .insert(Name::new("Floor"));

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
