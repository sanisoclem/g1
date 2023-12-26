use bevy::prelude::*;
use std::collections::HashMap;

use crate::AnimationController;

#[derive(Bundle)]
pub struct AnimatedBundle<T: Asset + AnimationController> {
  pub animator: Animator<T>,
  pub params: AnimationControllerInput,
  pub data: AnimationControllerData<T>,
}

impl<T: Asset + AnimationController> Default for AnimatedBundle<T>
where
  T::ControllerData: Default,
{
  fn default() -> Self {
    Self {
      animator: Animator::<T>::default(),
      params: AnimationControllerInput::default(),
      data: AnimationControllerData::default(),
    }
  }
}

#[derive(Component, Reflect)]
pub struct Animator<T: Asset> {
  pub controller: Handle<T>,
  pub rig_path: Option<EntityPath>,
}
impl<T: Asset> Default for Animator<T> {
  fn default() -> Self {
    Self {
      controller: Handle::default(),
      rig_path: None,
    }
  }
}

#[derive(Component, Default, Reflect)]
pub struct AnimatorTarget {
  pub rig_target: Option<Entity>,
}
#[derive(Component, Default, Reflect)]
pub struct AnimationControllerInput {
  pub(crate) parameters: HashMap<String, f32>,
}

#[derive(Component, Reflect)]
pub struct AnimationControllerData<T: AnimationController> {
  pub(crate) data: T::ControllerData,
}

impl AnimationControllerInput {
  pub fn set_parameter(&mut self, key: &'static str, value: f32) {
    if let Some(v) = self.parameters.get_mut(key) {
      if v != &value {
        *v = value
      }
    } else {
      self.parameters.insert(key.to_owned(), value);
    }
  }
}
impl<T: AnimationController> Default for AnimationControllerData<T>
where
  T::ControllerData: Default,
{
  fn default() -> Self {
    Self {
      data: T::ControllerData::default(),
    }
  }
}
