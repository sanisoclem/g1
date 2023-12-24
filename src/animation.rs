use assets::RonAsset;
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc, time::Duration};

use bevy::{asset::LoadContext, prelude::*};

#[derive(Component)]
pub struct Animator {
  pub controller: Handle<AnimationController>,
  pub parameters: HashMap<&'static str, f32>,
}

impl Animator {
  pub fn set_parameter(&mut self, key: &'static str, value: f32) {
    if let Some(v) = self.parameters.get_mut(key) {
      if v != &value {
        *v = value
      }
    } else {
      self.parameters.insert(key, value);
    }
  }
}

#[derive(Deserialize, Asset, TypePath)]
pub enum AnimationController {
  BasicAnimationController {
    nodes: HashMap<BasicNodeId, BasicAnimationNode>,
    edges: Vec<BasicAnimationTransition>,
    default_node: BasicNodeId,
    #[serde(skip_deserializing)]
    active_node: Option<BasicNodeId>, // TODO: modify the component, not the asset
    #[serde(skip_deserializing)]
    assets: Option<AnimationControllerAssets>,
  },
}
impl AnimationController {
  pub fn update_animation(&mut self, parameters: &HashMap<&str, f32>, trigger: Option<&str>) {
    let AnimationController::BasicAnimationController {
      ref mut active_node,
      nodes: _,
      edges,
      default_node,
      assets: _,
    } = self;

    let Some(active_node_id) = active_node else {
      *active_node = Some(default_node.clone());
      return;
    };

    let Some(transition) = edges
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
      return;
    };

    *active_node = Some(transition.to.clone());
  }
  pub fn get_active_animation(&self) -> Option<Handle<AnimationClip>> {
    let AnimationController::BasicAnimationController {
      active_node,
      nodes: _,
      edges: _,
      default_node: _,
      assets,
    } = self;
    let Some(assets) = assets else {
      return None;
    };
    let Some(active_node_id) = active_node else {
      return None;
    };
    let Some(anim) = assets.animations.get(active_node_id) else {
      return None;
    };

    return Some(anim.clone());
  }
}

pub struct AnimationControllerAssets {
  pub animations: HashMap<BasicNodeId, Handle<AnimationClip>>,
}

#[derive(PartialEq, Hash, Eq, Debug, Deserialize, Clone)]
pub struct BasicNodeId(Arc<String>);

#[derive(Deserialize)]
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
  pub speed: f32,
}

#[derive(Deserialize)]
pub enum BasicAnimationTransitionCondition {
  GreaterThan(String, f32),
  LessThan(String, f32),
  Trigger(String),
}

impl RonAsset for AnimationController {
  type NestedAssets = AnimationControllerAssets;
  fn construct_nested_assets<'a>(&mut self, load_context: &'a mut LoadContext) {
    let AnimationController::BasicAnimationController {
      nodes,
      edges: _,
      default_node: _,
      ref mut assets,
      active_node: _,
    } = self;
    *assets = Some(AnimationControllerAssets {
      animations: nodes
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
    &["anim.ron"]
  }
}

pub fn play_animations(
  mut controllers: ResMut<Assets<AnimationController>>,
  mut qry: Query<(&Animator, &mut AnimationPlayer), Changed<Animator>>,
) {
  for (animator, mut player) in qry.iter_mut() {
    let Some(controller) = controllers.get_mut(animator.controller.clone()) else {
      continue;
    };
    controller.update_animation(&animator.parameters, None);
    if let Some(anim) = controller.get_active_animation() {
      // TODO: get speed, transition time and repeat mode
      player.play_with_transition(anim, Duration::from_secs_f32(0.5));
    } else {
      player.pause()
    }
  }
}
