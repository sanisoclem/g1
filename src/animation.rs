use assets::RonAsset;
use serde::Deserialize;
use std::{collections::HashMap, ops::Deref, sync::Arc, time::Duration};

use bevy::{asset::LoadContext, prelude::*, scene::SceneInstance};

#[derive(Component, Default)]
pub struct Animator {
  pub controller: Handle<AnimationController>,
  pub parameters: HashMap<&'static str, f32>,
  pub rig_path: EntityPath,
  pub rig_calculated: bool,
  pub rig_target: Option<Entity>,
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

pub fn find_rig_target(
  mut qry: Query<(Entity, &mut Animator), (With<Children>, With<SceneInstance>)>,
  children: Query<&Children>,
  names: Query<&Name>,
) {
  for (e, mut animator) in qry.iter_mut() {
    if animator.rig_calculated {
      continue;
    };
    let mut cache = Vec::new();
    animator.rig_target = entity_from_path2(e, &animator.rig_path, &children, &names, &mut cache);
    animator.rig_calculated = true;
  }
}

pub fn play_animations(
  mut controllers: ResMut<Assets<AnimationController>>,
  qry: Query<&Animator, Changed<Animator>>,
  mut qry_player: Query<&mut AnimationPlayer>,
) {
  for animator in qry.iter() {
    let Some(rig_target) = animator.rig_target else {
      continue;
    };
    let Ok(mut player) = qry_player.get_mut(rig_target) else {
      continue;
    };
    let Some(controller) = controllers.get_mut(animator.controller.clone()) else {
      continue;
    };

    controller.update_animation(&animator.parameters, None);
    if let Some(anim) = controller.get_active_animation() {
      // TODO: get speed, transition time and repeat mode
      player.play_with_transition(anim, Duration::from_secs_f32(0.5));
      player.set_repeat(bevy::animation::RepeatAnimation::Forever);
    } else {
      player.pause()
    }
  }
}

fn entity_from_path2(
  root: Entity,
  path: &EntityPath,
  children: &Query<&Children>,
  names: &Query<&Name>,
  path_cache: &mut Vec<Option<Entity>>,
) -> Option<Entity> {
  // PERF: finding the target entity can be optimised
  let mut current_entity = root;
  path_cache.resize(path.parts.len(), None);

  let parts = path.parts.iter().enumerate();

  for (idx, part) in parts {
    let mut found = false;
    let Ok(children) = children.get(current_entity) else {
      warn!(
        "Cannot find rig target {:?}. no children found for {:?}",
        path, current_entity
      );
      return None;
    };
    if let Some(cached) = path_cache[idx] {
      if children.contains(&cached) {
        if let Ok(name) = names.get(cached) {
          if name == part {
            current_entity = cached;
            found = true;
          }
        }
      }
    }
    if !found {
      for child in children.deref() {
        let name = names.get(*child).unwrap_or(part);

        if name == part {
          current_entity = *child;
          path_cache[idx] = Some(*child);
          found = true;
          break;
        }
      }
    }
    if !found {
      warn!("Entity not found for path {:?} on part {:?}", path, part);
      return None;
    }
  }
  Some(current_entity)
}
