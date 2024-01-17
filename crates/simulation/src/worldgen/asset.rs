use super::{WorldLayout, WorldSeed};
use assets::RonAsset;
use bevy::prelude::*;
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, TypePath, Deserialize)]
pub struct WorldBlueprint<T: WorldLayout> {
  pub name: String,
  pub layout: T,
}

impl<T: WorldLayout> RonAsset for WorldBlueprint<T> {
  type NestedAssets = ();
  fn construct_nested_assets<'a>(&mut self, _load_context: &'a mut bevy::asset::LoadContext) {
    ()
  }
  fn extensions() -> &'static [&'static str] {
    &["world.ron"]
  }
}

pub trait WorldChunkLayerAsset<T: WorldLayout>: RonAsset {
  fn generate(seed: &WorldSeed, chunk_id: &T::ChunkId) -> Result<String, WorldGenerationError>;
}

#[derive(Error, Debug)]
pub enum WorldGenerationError {
  #[error("ron error")]
  RonError {
    #[from]
    source: serde_ron::Error,
  },
  #[error("io error")]
  IoError {
    #[from]
    source: std::io::Error,
  },
  #[error("unknown error")]
  Unknown,
}
