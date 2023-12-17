use bevy::prelude::*;

pub mod movement;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(
        FixedUpdate,
        (movement::move_moveables, movement::update_move_requests),
      )
      .insert_resource(Time::<Fixed>::from_seconds(1. / 60.));
  }
}
