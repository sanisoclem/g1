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

fn main() {
  let mut app = App::new();
  app
    .insert_resource(ClearColor(Color::BLACK))
    // .insert_resource(AssetMetaCheck::Never) // might need this wasm hosting (itch.io returns 403s and loader panics)
    .add_plugins((DefaultPlugins, TemporalAntiAliasPlugin, SimulationPlugin))
    .add_systems(Startup, setup)
    .add_systems(Update, (controls, update_camera));

  #[cfg(feature = "debug")]
  app.add_plugins((EguiPlugin, utils::fps::ScreenDiagsTextPlugin));

  app.run();
}

fn controls(
  mut qry: Query<&mut Transform, With<Player>>,
  mut camera_query: Query<(&Camera, &GlobalTransform)>,
  mut gizmos: Gizmos,
  keyboard_input: Res<Input<KeyCode>>,
  time: Res<Time>,
) {
  let Ok(mut trans) = qry.get_single_mut() else {
    return;
  };
  let Ok((camera, cam_gtransform)) = camera_query.get_single() else {
    return;
  };

  let mut f = cam_gtransform.forward();
  f.y = 0.0;

  let angle = Quat::from_rotation_y(f.normalize().angle_between(Vec3::X));

  let mut dir = Vec3::ZERO;
  if keyboard_input.pressed(KeyCode::D) {
    dir += Vec3::Z;
  }
  if keyboard_input.pressed(KeyCode::S) {
    dir += Vec3::NEG_X;
  }
  if keyboard_input.pressed(KeyCode::A) {
    dir += Vec3::NEG_Z;
  }
  if keyboard_input.pressed(KeyCode::W) {
    dir += Vec3::X;
  }

  dir = angle.mul_vec3(dir);

  gizmos.line(Vec3::X * -100., Vec3::X * 100., Color::BLUE);
  gizmos.line(Vec3::Y * -100., Vec3::Y * 100., Color::GREEN);
  gizmos.line(Vec3::Z * -100., Vec3::Z * 100., Color::RED);

  gizmos.line(Vec3::ZERO, dir * 100., Color::PURPLE);

  let inc = dir *10.* time.delta_seconds();
  trans.translation.x += inc.x;
  trans.translation.z += inc.z;
}

fn update_camera(
  qry: Query<&Transform, (With<Player>, Without<Camera>)>,
  mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
  time: Res<Time>,
) {
  let Ok(player_trans) = qry.get_single().map(|a| a.clone()) else {
    return;
  };
  let Ok(mut cam_transform) = camera_query.get_single_mut() else {
    return;
  };
  let desired_pos = player_trans.translation + Vec3::splat(5.0);
  cam_transform.translation = lerp(cam_transform.translation, desired_pos, time.delta_seconds());
  cam_transform.look_at(player_trans.translation, Vec3::Y);
}

#[derive(Component)]
pub struct Player;

fn setup(
  mut cmd: Commands,
  asset_server: Res<AssetServer>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut meshes: ResMut<Assets<Mesh>>,
) {
  let skybox_handle = asset_server.load("textures/Ryfjallet_cubemap_astc4x4.ktx2");
  cmd
    .spawn((
      Camera3dBundle {
        camera: Camera {
          hdr: true, // 1. HDR is required for bloom
          ..default()
        },
        projection: OrthographicProjection {
          scale: 3.0,
          scaling_mode: ScalingMode::FixedVertical(2.),
          ..default()
        }
        .into(),
        tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
      },
      Skybox(skybox_handle.clone()),
      BloomSettings::default(),
      // FogSettings {
      //   color: Color::rgba(0.35, 0.48, 0.66, 1.0),
      //   directional_light_color: Color::rgba(1.0, 0.95, 0.85, 0.5),
      //   directional_light_exponent: 30.0,
      //   falloff: FogFalloff::from_visibility_colors(
      //     50.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
      //     Color::rgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
      //     Color::rgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
      //   ),
      // },
    ))
    .insert(ScreenSpaceAmbientOcclusionBundle::default());
  // .insert(TemporalAntiAliasBundle::default());

  cmd
    .spawn(SceneBundle {
      scene: asset_server.load("char.glb#Scene0"),
      ..default()
    })
    .insert(Player);

  // plane
  cmd.spawn(PbrBundle {
    mesh: meshes.add(shape::Plane::from_size(500.0).into()),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    ..default()
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
