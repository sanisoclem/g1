use assets::RonAssetApp;
use bevy::prelude::*;

#[derive(Default)]
pub struct AnimationControllerPlugin;

impl Plugin for AnimationControllerPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_ron_asset::<BasicAnimationController>()
      .register_type::<Animator<BasicAnimationController>>()
      .register_type::<AnimatorTarget>()
      .register_type::<AnimationControllerInput>()
      .register_type::<AnimationControllerData<BasicAnimationController>>()
      .add_systems(
        Update,
        ((
          find_rig_target::<BasicAnimationController>,
          play_animations::<BasicAnimationController>,
        )
          .chain(),),
      );
  }
}

mod animator;
mod basic_controller;
mod controller;

pub use animator::{
  AnimatedBundle, AnimationControllerData, AnimationControllerInput, Animator, AnimatorTarget,
};
pub use basic_controller::BasicAnimationController;
pub use controller::AnimationController;
use controller::{find_rig_target, play_animations};
