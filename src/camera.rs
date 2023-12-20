use std::ops::Mul;

use bevy::{
  core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping, Skybox},
  pbr::ScreenSpaceAmbientOcclusionBundle,
  prelude::*,
  render::camera::ScalingMode,
};

use utils::lerp;
#[derive(Component)]
pub struct CameraTarget;

pub fn update_camera(
  qry: Query<&Transform, (With<CameraTarget>, Without<Camera>)>,
  mut camera_query: Query<&mut Transform, (With<Camera>, Without<CameraTarget>)>,
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

pub fn setup_camera(mut cmd: Commands, asset_server: Res<AssetServer>) {
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
}
