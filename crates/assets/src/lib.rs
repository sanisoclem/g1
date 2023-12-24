use serde::Deserialize;
use serde_ron::de::from_bytes;
use std::marker::PhantomData;
use thiserror::Error;

use bevy::{
  asset::{io::Reader, AssetApp, AssetLoader, AsyncReadExt, LoadContext},
  prelude::*,
  utils::BoxedFuture,
};

pub trait RonAssetApp {
  fn register_ron_asset<A: RonAsset>(&mut self) -> &mut Self
  where
    A: for<'a> Deserialize<'a> + RonAsset + Asset + Send + Sync + 'static;
}

impl RonAssetApp for App {
  fn register_ron_asset<A: RonAsset>(&mut self) -> &mut Self
  where
    A: for<'a> Deserialize<'a> + RonAsset + Asset + Send + Sync + 'static,
  {
    self.init_asset_loader::<RonAssetLoader<A>>()
  }
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum RonAssetLoaderError {
  #[error("Could not load asset: {0}")]
  Io(#[from] std::io::Error),
  #[error("Could not parse RON: {0}")]
  RonSpannedError(#[from] serde_ron::error::SpannedError),
}

pub trait RonAsset {
  type NestedAssets;

  fn construct_nested_assets<'a>(&mut self, load_context: &'a mut LoadContext);
  fn extensions() -> &'static [&'static str];
}

pub struct RonAssetLoader<T> {
  phantom: PhantomData<T>,
}

impl<T> Default for RonAssetLoader<T> {
  fn default() -> Self {
    RonAssetLoader {
      phantom: PhantomData,
    }
  }
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
      let mut asset = from_bytes::<T>(&bytes)?;
      asset.construct_nested_assets(ctx);

      Ok(asset)
    })
  }

  fn extensions(&self) -> &[&str] {
    T::extensions()
  }
}
