use assets::RonAsset;
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};

use bevy::{asset::LoadContext, prelude::*};

#[derive(Component)]
pub struct Animator {
  pub controller: Handle<AnimationController>,
  pub parameters: HashMap<&'static str, f32>,
}

#[derive(Deserialize, Asset, TypePath)]
pub enum AnimationController {
  BasicAnimationController {
    nodes: HashMap<BasicNodeId, BasicAnimationNode>,
    edges: Vec<BasicAnimationTransition>,
    default_node: BasicNodeId,
    #[serde(skip_deserializing)]
    assets: Option<AnimationControllerAssets>, // layers: u8, // blend/mask animations?
  },
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
