use std::{any::TypeId, marker::PhantomData, sync::Arc};

use bevy::{
  prelude::*,
  tasks::{block_on, AsyncComputeTaskPool, Task},
  utils::HashMap,
};
use futures_lite::future;

#[derive(PartialEq, Hash, Eq, Clone, Default)]
pub struct ChunkId(u16, u16);

#[derive(PartialEq, Hash, Eq, Clone)]
pub struct ChunkLayerId(ChunkId, TypeId);

#[derive(PartialEq, Hash, Eq, Clone)]
pub struct WorldSeed(Arc<[u8; 16]>);

#[derive(Asset, TypePath)]
pub struct WorldBlueprint {
  pub name: String,
  pub height: u16,
  pub radius: u16,
  pub chunk_size: f32,
}

#[derive(Resource)]
pub struct WorldInstance {
  pub blueprint: Handle<WorldBlueprint>,
  pub seed: WorldSeed,
  pub chunk_cache: HashMap<ChunkLayerId, String>, // procedurally generated data only
}

impl WorldInstance {
  pub fn set_chunk_cache<T>(&mut self, chunk_id: &ChunkId, path: String) {
    unimplemented!()
  }
}

pub trait WorldChunkLayerAsset: Asset {
  type PersistAs;
  fn generate(seed: &WorldSeed, chunk_id: &ChunkId) -> Self::PersistAs;
}

#[derive(Component, Default)]
pub struct WorldChunk {
  pub chunk_id: ChunkId,
  // pub layers: HashMap<TypeId, Entity>,
  pub lod: u8,
}
#[derive(Component, Default)]
pub struct WorldChunkLayer<T: WorldChunkLayerAsset> {
  pub data: Handle<T>,
}

#[derive(Bundle, Default)]
pub struct WorldChunkBundle {
  pub chunk: WorldChunk,
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

#[derive(Resource)]
pub struct WorldManager {
  pub generating_chunks: HashMap<ChunkLayerId, Entity>,
  pub loaded_chunks: HashMap<ChunkId, Entity>,
}

impl WorldManager {
  pub fn mark_chunk_as_loaded(&mut self, chunk_id: &ChunkId, entity: Entity) {
    if self.loaded_chunks.contains_key(chunk_id) {
      panic!("Attempted to load chunk twice")
    }
    self.loaded_chunks.insert(chunk_id.clone(), entity);
  }
  pub fn mark_chunk_layer_as_generating<T: 'static>(
    &mut self,
    chunk_id: &ChunkId,
    entity: Entity,
    generated: bool,
  ) {
    let key = ChunkLayerId(chunk_id.clone(), TypeId::of::<T>());
    if generated {
      self.generating_chunks.remove(&key);
    } else {
      if self.generating_chunks.contains_key(&key) {
        panic!("Attempted to generate chunk twice")
      }
      self.generating_chunks.insert(key, entity);
    }
  }
  pub fn get_chunks_to_generate(&self, instance: &WorldInstance) -> Vec<ChunkId> {
    unimplemented!()
  }
  pub fn get_chunks_to_spawn(&self, instance: &WorldInstance) -> Vec<ChunkId> {
    unimplemented!()
  }
  pub fn get_chunks_to_despawn(&self) -> Vec<ChunkId> {
    unimplemented!()
  }
}

#[derive(Resource)]
pub struct WorldSettings {
  pub max_loaded_chunks: u16,
}

#[derive(Component)]
pub struct WorldChunkGenerationTask<T: Send + Sync>(Task<(ChunkId, String, PhantomData<T>)>);

#[derive(Component)]
pub struct WorldChunkLoadingTask<T: WorldChunkLayerAsset>(Task<(ChunkId, T::PersistAs)>);

pub fn generate_chunk_layers<T: Send + Sync + WorldChunkLayerAsset + 'static>(
  mut cmd: Commands,
  world_instance: Option<Res<WorldInstance>>,
  mut world_manager: ResMut<WorldManager>,
) {
  let Some(current_world) = world_instance else {
    return;
  };

  let thread_pool = AsyncComputeTaskPool::get();

  for chunk_id in world_manager.get_chunks_to_generate(&current_world) {
    let entity = cmd.spawn_empty().id();
    world_manager.mark_chunk_layer_as_generating::<T>(&chunk_id, entity.clone(), false);

    let seed = current_world.seed.clone();

    let task = thread_pool.spawn(async move {
      let _data = T::generate(&seed, &chunk_id);
      // TODO: persist to disk
      let file = "TODO";
      (chunk_id, file.to_owned(), PhantomData)
    });

    cmd
      .entity(entity)
      .insert(WorldChunkGenerationTask::<T>(task));
  }
}

pub fn poll_chunk_layer_generation_tasks<T: Send + Sync + 'static>(
  mut cmd: Commands,
  mut tasks: Query<(Entity, &mut WorldChunkGenerationTask<T>)>,
  mut world_manager: ResMut<WorldManager>,
  world_instance: Option<ResMut<WorldInstance>>,
) {
  let Some(mut current_world) = world_instance else {
    return;
  };

  for (entity, mut task) in &mut tasks {
    if let Some((chunk_id, file_name, _)) = block_on(future::poll_once(&mut task.0)) {
      world_manager.mark_chunk_layer_as_generating::<T>(&chunk_id, entity.clone(), true);
      current_world.set_chunk_cache::<T>(&chunk_id, file_name);
      cmd.entity(entity).despawn_recursive();
    }
  }
}

pub fn spawn_chunk(
  mut cmd: Commands,
  mut world_manager: ResMut<WorldManager>,
  world_instance: Option<ResMut<WorldInstance>>,
) {
  let Some(current_world) = world_instance else {
    return;
  };

  for chunk_id in world_manager.get_chunks_to_spawn(&current_world) {
    let entity = cmd
      .spawn(WorldChunkBundle {
        chunk: WorldChunk {
          chunk_id: chunk_id.clone(),
          ..default()
        },
        ..default()
      })
      .id();

    world_manager.mark_chunk_as_loaded(&chunk_id, entity);
  }
}

pub fn spawn_chunk_layers<T: WorldChunkLayerAsset>(
  mut cmd: Commands,
  asset_server: AssetServer,
  world_instance: Option<ResMut<WorldInstance>>,
  qry: Query<(Entity, &WorldChunk), Added<WorldChunk>>,
) {
  let Some(current_world) = world_instance else {
    return;
  };

  for (entity, chunk) in qry.iter() {
    let key = ChunkLayerId(chunk.chunk_id.clone(), TypeId::of::<T>());
    let Some(generated_asset) = current_world.chunk_cache.get(&key) else {
      continue;
    };

    cmd.entity(entity).with_children(|b| {
      b.spawn(WorldChunkLayer {
        data: asset_server.load::<T>(generated_asset),
      });
    });
  }
}
