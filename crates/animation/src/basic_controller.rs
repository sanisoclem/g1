use assets::RonAsset;
use bevy::{asset::LoadContext, prelude::*};
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc, time::Duration};

use crate::{AnimationController, AnimationControllerInput};

#[derive(Deserialize, Asset, TypePath)]
pub struct BasicAnimationController {
  nodes: HashMap<BasicNodeId, BasicAnimationNode>,
  edges: Vec<BasicAnimationTransition>,
  default_node: BasicNodeId,
  #[serde(skip_deserializing)]
  assets: Option<BasicAnimationControllerAssets>,
}

#[derive(Default, Reflect)]
pub struct BasicAnimationControllerData {
  active_node: Option<BasicNodeId>,
}
impl BasicAnimationController {
  fn get_transition(
    &self,
    parameters: &HashMap<String, f32>,
    trigger: Option<&str>,
    data: &BasicAnimationControllerData,
  ) -> Option<BasicAnimationTransition> {
    let Some(active_node_id) = &data.active_node else {
      return Some(BasicAnimationTransition {
        to: self.default_node.clone(),
        ..default()
      });
    };

    let Some(transition) = self
      .edges
      .iter()
      .filter(|e| {
        if let Some(from) = &e.from {
          if active_node_id != from {
            return false;
          }
        }
        e.conditions.iter().all(|c| match c {
          BasicAnimationTransitionCondition::GreaterThan(name, v) => {
            parameters.get(name.as_str()).unwrap_or(&0.0) > v
          }
          BasicAnimationTransitionCondition::LessThan(name, v) => {
            parameters.get(name.as_str()).unwrap_or(&0.0) < v
          }
          BasicAnimationTransitionCondition::Trigger(t) => {
            if let Some(trigger) = trigger {
              t == trigger
            } else {
              false
            }
          }
        })
      })
      .nth(0)
    else {
      return None;
    };

    Some(transition.clone())
  }
}
impl AnimationController for BasicAnimationController {
  type ControllerData = BasicAnimationControllerData;
  fn update_animation(
    &self,
    parameters: &AnimationControllerInput,
    trigger: Option<&str>,
    data: &mut Self::ControllerData,
    player: &mut AnimationPlayer,
  ) {
    let Some(assets) = &self.assets else {
      warn!("Cannot compute transition, assets not found");
      return;
    };
    let Some(transition) = self.get_transition(&parameters.parameters, trigger, data) else {
      return;
    };
    let Some(node) = self.nodes.get(&transition.to) else {
      warn!(
        "Animation node {:?} not found, cannot execute transition",
        transition.to
      );
      return;
    };
    let Some(anim) = assets.animations.get(&transition.to) else {
      warn!(
        "Animation {:?} not found, cannot execute transition",
        transition.to
      );
      return;
    };

    data.active_node = Some(transition.to);
    player.play_with_transition(
      anim.clone(),
      Duration::from_secs_f32(transition.transition_duration_seconds),
    );
    player.set_speed(node.speed);
    if node.repeat {
      player.set_repeat(bevy::animation::RepeatAnimation::Forever);
    } else {
      player.set_repeat(bevy::animation::RepeatAnimation::Count(1));
    }
  }
}

pub struct BasicAnimationControllerAssets {
  pub animations: HashMap<BasicNodeId, Handle<AnimationClip>>,
}

#[derive(PartialEq, Hash, Eq, Debug, Deserialize, Clone, Default, Reflect)]
pub struct BasicNodeId(Arc<String>);

#[derive(Deserialize, Default, Clone)]
pub struct BasicAnimationTransition {
  pub from: Option<BasicNodeId>, // any node if node
  pub to: BasicNodeId,
  pub transition_duration_seconds: f32,
  pub enabled: bool,
  pub conditions: Vec<BasicAnimationTransitionCondition>,
}

#[derive(Deserialize)]
pub struct BasicAnimationNode {
  pub animation: String,
  pub repeat: bool,
  pub speed: f32,
}

#[derive(Deserialize, Clone)]
pub enum BasicAnimationTransitionCondition {
  GreaterThan(String, f32),
  LessThan(String, f32),
  Trigger(String),
}

impl RonAsset for BasicAnimationController {
  type NestedAssets = BasicAnimationControllerAssets;
  fn construct_nested_assets<'a>(&mut self, load_context: &'a mut LoadContext) {
    self.assets = Some(BasicAnimationControllerAssets {
      animations: self
        .nodes
        .iter()
        .map(|(node_id, node)| {
          (
            node_id.clone(),
            load_context.load::<AnimationClip>(&node.animation),
          )
        })
        .collect::<HashMap<_, _>>(),
    });
  }
  fn extensions() -> &'static [&'static str] {
    &["basic.anim.ron"]
  }
}
