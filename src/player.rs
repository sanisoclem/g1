use bevy::prelude::*;

use crate::animation::Animator;

#[derive(Component)]
pub struct Player;

pub fn setup_player(
  mut cmd: Commands,
  asset_server: Res<AssetServer>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut meshes: ResMut<Assets<Mesh>>,
) {
  cmd
    .spawn(SceneBundle {
      scene: asset_server.load("char.glb#Scene0"),
      ..default()
    })
    .insert(Player)
    .insert(Animator {
      controller: asset_server.load("player.anim.ron"),
      parameters: vec![("velocity", 0.0)].into_iter().collect(),
    });

  // plane
  cmd.spawn(PbrBundle {
    mesh: meshes.add(shape::Plane::from_size(500.0).into()),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    ..default()
  });
}

pub fn update_player(
  mut qry: Query<(&mut Transform, &mut Animator), With<Player>>,
  mut gizmos: Gizmos,
  keyboard_input: Res<Input<KeyCode>>,
  time: Res<Time>,
) {
  let Ok((mut trans, mut animator)) = qry.get_single_mut() else {
    return;
  };

  let angle = Quat::from_rotation_y(45.0f32.to_radians());

  let mut dir = Vec3::ZERO;
  if keyboard_input.pressed(KeyCode::D) {
    dir += Vec3::X;
  }
  if keyboard_input.pressed(KeyCode::S) {
    dir += Vec3::Z;
  }
  if keyboard_input.pressed(KeyCode::A) {
    dir += Vec3::NEG_X;
  }
  if keyboard_input.pressed(KeyCode::W) {
    dir += Vec3::NEG_Z;
  }

  dir = angle.mul_vec3(dir);

  gizmos.line(Vec3::X * 0., Vec3::X * 100., Color::BLUE);
  gizmos.line(Vec3::Y * 0., Vec3::Y * 100., Color::GREEN);
  gizmos.line(Vec3::Z * 0., Vec3::Z * 100., Color::RED);

  gizmos.line(Vec3::ZERO, dir * 100., Color::PURPLE);

  if dir == Vec3::ZERO {
    animator.set_parameter("velocity", 0.0);
    return;
  }

  animator.set_parameter("velocity", 1.0);
  let inc = dir * 5. * time.delta_seconds();
  let plus1s = Vec3::new(
    trans.translation.x - inc.x,
    trans.translation.y,
    trans.translation.z - inc.z,
  );
  trans.look_at(plus1s, Vec3::Y);
  trans.translation.x += inc.x;
  trans.translation.z += inc.z;
}
