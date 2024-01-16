use super::{asset::WorldChunkLayerAsset, layout::WorldLayout};
use bevy::{prelude::*, tasks::Task};
use std::marker::PhantomData;

#[derive(Component, Reflect)]
pub struct WorldChunkGenerationTask<T: Send + Sync, TChunk>(
  pub(crate) Task<(TChunk, String, PhantomData<T>)>,
);

// #[derive(Component)]
// pub struct WorldChunkLoadingTask<T: WorldChunkLayerAsset<A>, A: WorldLayout>(
//   pub(crate) Task<(ChunkId, T::PersistAs)>,
// );

#[derive(Bundle, Default)]
pub struct WorldChunkBundle<T: 'static + Send + Sync + Default> {
  pub chunk: WorldChunk<T>,
  pub transform: Transform,
  pub global_transform: GlobalTransform,
  /// User indication of whether an entity is visible
  pub visibility: Visibility,
  /// Inherited visibility of an entity.
  pub inherited_visibility: InheritedVisibility,
  /// Algorithmically-computed indication of whether an entity is visible and should be extracted
  /// for rendering
  pub view_visibility: ViewVisibility,
}

#[derive(Component)]
pub struct WorldLoadMarker;

#[derive(Component, Default, Reflect)]
pub struct WorldChunk<T: Default> {
  pub chunk_id: T,
  pub lod: u16,
}

#[derive(Component, Default, Reflect)]
pub struct WorldChunkLayer<T: WorldChunkLayerAsset<A>, A: WorldLayout> {
  pub data: Handle<T>,
  pub phatom: PhantomData<A>,
}
