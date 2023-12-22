use serde::Deserialize;
use serde_ron::de::from_bytes;
use std::{collections::HashMap, marker::PhantomData, time::Duration};
use thiserror::Error;

use bevy::{
  asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
  prelude::*,
  utils::BoxedFuture,
};

#[derive(Component)]
pub struct Animator {
  pub controller: Handle<AnimationController>,
  pub parameters: HashMap<&'static str, f32>,
}

#[derive(
  serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath, custom_derive::RonAsset,
)]
#[ron_asset(extension = "anim.ron", assets = AnimationControllerAssets)]
pub enum AnimationController {
  NoobAnimationController(NoobAnimationController),
}
pub struct AnimationControllerAssets {
  animations: HashMap<NoobNodeId, Handle<AnimationClip>>,
}

#[derive(PartialEq, Hash, Eq, Debug, serde::Deserialize)]
pub struct NoobNodeId(u16);

#[derive(serde::Deserialize)]
pub struct NoobAnimationController {
  pub nodes: HashMap<NoobNodeId, NoobAnimationNode>,
  pub edges: Vec<NoobAnimationTransition>,
  pub default_node: NoobNodeId,
  pub layers: u8, // blend/mask animations?
}

#[derive(serde::Deserialize)]
pub struct NoobAnimationTransition {
  pub from: Option<NoobNodeId>, // any node if node
  pub to: NoobNodeId,
  pub transition_duration: Duration,
  pub enabled: bool,
  pub conditions: Vec<NoobAnimationTransitionCondition>,
}

#[derive(serde::Deserialize)]
pub struct NoobAnimationNode {
  pub animation: String,
  pub speed: f32,
}

#[derive(serde::Deserialize)]
pub enum NoobAnimationTransitionCondition {
  GreaterThan(String, f32),
  LessThan(String, f32),
  Trigger(String),
}

/// Possible errors that can be produced by [`CustomAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum RonAssetLoaderError {
  /// An [IO](std::io) Error
  #[error("Could not load asset: {0}")]
  Io(#[from] std::io::Error),
  /// A [RON](ron) Error
  #[error("Could not parse RON: {0}")]
  RonSpannedError(#[from] serde_ron::error::SpannedError),
}

pub trait RonAsset {
  type NestedAssets;

  fn construct_nested_assets<'a>(&self, load_context: &'a mut LoadContext) -> Self::NestedAssets;
  fn extensions() -> &'static [&'static str];
}

impl RonAsset for AnimationController {
  type NestedAssets = AnimationControllerAssets;
  fn construct_nested_assets<'a>(&self, load_context: &'a mut LoadContext) -> Self::NestedAssets {
    unimplemented!()
  }
  fn extensions() -> &'static [&'static str] {
    &["anim.ron"]
  }
}

#[derive(Default)]
pub struct RonAssetLoader<T> {
  phantom: PhantomData<T>,
}

impl<T> AssetLoader for RonAssetLoader<T>
where
  T: for<'a> Deserialize<'a> + RonAsset + Asset + Send + Sync + 'static,
{
  type Asset = T;
  type Settings = ();
  type Error = RonAssetLoaderError;

  fn load<'a>(
    &'a self,
    reader: &'a mut Reader,
    _settings: &'a (),
    ctx: &'a mut LoadContext,
  ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
    Box::pin(async move {
      let mut bytes = Vec::new();
      reader.read_to_end(&mut bytes).await?;
      let asset = from_bytes::<T>(&bytes)?;
      asset.construct_nested_assets(ctx);

      Ok(asset)
    })
  }

  fn extensions(&self) -> &[&str] {
    T::extensions()
  }
}
