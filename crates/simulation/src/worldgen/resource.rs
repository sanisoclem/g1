use super::{asset::WorldBlueprint, layout::WorldLayout, WorldSeed};
use bevy::{prelude::*, utils::hashbrown::HashMap};
use std::any::TypeId;

/// Identifies a WorkdChunkLayer.
///
/// The [`TypeId`] is from the type that implements [`WorldChunkLayerAsset`].
#[derive(PartialEq, Hash, Eq, Clone)]
pub struct ChunkLayerId<T>(pub(crate) T, pub(crate) TypeId);

/// Stores output of world generation. This data is designed to be
/// persisted, but can be regenerated.
/// TODO:
///  - persist this data
///  - load data
#[derive(Resource, Default)]
pub struct WorldGenerationCache<TChunkId> {
  // TODO: add finger print to ensure cache matches blueprint
  pub chunk_cache: HashMap<ChunkLayerId<TChunkId>, String>,
}

impl<TChunkId> WorldGenerationCache<TChunkId>
where
  TChunkId: std::hash::Hash + PartialEq + Eq + Clone,
{
  pub fn set_chunk_cache<T: 'static>(&mut self, chunk_id: &TChunkId, path: String) {
    self.chunk_cache.insert(
      ChunkLayerId(chunk_id.clone(), TypeId::of::<T>()),
      path.clone(),
    );
  }
}

/// Stores the current runtime state of the world
#[derive(Resource)]
pub struct WorldManager<T: WorldLayout> {
  pub current: Option<(Handle<WorldBlueprint<T>>, WorldSeed)>,
  pub origin: T::ChunkId,
  pub pending_generation: HashMap<ChunkLayerId<T::ChunkId>, Entity>,
  /// Tracks all chunk entities that have been spawned
  /// in theory, despawning these entities should be enough to unload
  /// a chunk since all entities spawned by each layer will be a child
  /// of the chunk entity
  pub loaded_chunks: HashMap<T::ChunkId, Entity>,
}

impl<T> Default for WorldManager<T>
where
  T: WorldLayout,
{
  fn default() -> Self {
    Self {
      current: None,
      origin: T::ChunkId::default(),
      pending_generation: HashMap::new(),
      loaded_chunks: HashMap::new(),
    }
  }
}

impl<T> WorldManager<T>
where
  T: WorldLayout,
  <T as WorldLayout>::ChunkId: Clone + std::hash::Hash + PartialEq + Eq,
{
  /// A loaded chunk means the entity for that chunk has been spawned.
  /// Chunk layer entities may or may not have been spawned
  ///
  /// # Panics
  ///
  /// The function will panic if the chunk is already marked as loaded, as this might mean
  /// that multiple entities were spawned for the chunk.
  pub fn mark_chunk_as_loaded(&mut self, chunk_id: &T::ChunkId, entity: Entity) {
    if self.loaded_chunks.contains_key(chunk_id) {
      panic!("Attempted to load chunk twice")
    }
    self.loaded_chunks.insert(chunk_id.clone(), entity);
  }

  pub fn mark_chunk_layer_as_generating<V: 'static>(
    &mut self,
    chunk_id: &T::ChunkId,
    entity: Entity,
    generated: bool,
  ) {
    let key = ChunkLayerId(chunk_id.clone(), TypeId::of::<V>());
    if generated {
      self.pending_generation.remove(&key);
    } else {
      if self.pending_generation.contains_key(&key) {
        panic!("Attempted to generate chunk twice")
      }
      self.pending_generation.insert(key, entity);
    }
  }

  // pub fn mark_chunk_as_unloaded(&mut self, chunk_id: &T::ChunkId) {
  //   unimplemented!()
  // }
}

#[derive(Resource, Default)]
pub struct WorldSettings {
  pub generate_lod_threshold: u16,
  pub visibility_lod_threshold: u16,
  pub visibility_lod_overlap: u16,
}
