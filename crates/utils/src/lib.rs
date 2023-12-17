use std::ops::{Add, Mul, Sub};

use bevy::prelude::*;
pub mod fps;

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
  for entity in &to_despawn {
    commands.entity(entity).despawn_recursive();
  }
}

pub fn lerp<T: Copy + Mul<f32, Output = T> + Sub<T, Output = T> + Add<T, Output = T>>(
  from: T,
  to: T,
  f: f32,
) -> T {
  from + ((to - from) * f)
}
