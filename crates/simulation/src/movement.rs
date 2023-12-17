use bevy::prelude::*;

#[derive(Component)]
pub struct SimulationTransform {
  pub translation: Vec2,
  pub rotation: Mat2,
  pub z_index: f32,
}

#[derive(Component)]
pub struct Moveable {
  pub velocity: Vec2,
}

#[derive(Component)]
pub struct MoveRequest {
  pub translation: Option<Vec2>,
  pub rotation: Option<Mat2>,
}

pub(crate) fn move_moveables(
  mut qry: Query<(&mut SimulationTransform, &Moveable), Without<MoveRequest>>,
  time: Res<Time>,
) {
  for (mut transform, moveable) in qry.iter_mut() {
    if moveable.velocity == Vec2::ZERO {
      continue;
    }
    let inc = transform.rotation.mul_vec2(moveable.velocity) * time.delta_seconds();
    transform.translation += inc;
  }
}

pub(crate) fn update_move_requests(mut qry: Query<(&mut SimulationTransform, &mut MoveRequest)>) {
  for (mut transform, mut req) in qry.iter_mut() {
    if let Some(position) = req.translation {
      // TODO: check bounds etc
      transform.translation = position;
      req.translation = None;
    }

    if let Some(rot) = req.rotation {
      transform.rotation = rot;
      req.rotation = None;
    }
  }
}
