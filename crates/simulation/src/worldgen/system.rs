use bevy::{
  prelude::*,
  tasks::{block_on, AsyncComputeTaskPool},
};
use futures_lite::future;
use std::{any::TypeId, collections::HashSet, marker::PhantomData};

use super::{
  asset::{WorldBlueprint, WorldChunkLayerAsset},
  component::{
    WorldChunk, WorldChunkBundle, WorldChunkGenerationTask, WorldChunkLayer, WorldLoadMarker,
  },
  layout::WorldLayout,
  resource::{ChunkLayerId, WorldGenerationCache, WorldManager, WorldSettings},
  WorldCommand,
};

pub fn handle_world_commands<T: WorldLayout>(
  mut cmds: EventReader<WorldCommand>,
  mut cmd: Commands,
  asset_server: Res<AssetServer>,
  mut world_mgr: ResMut<WorldManager<T>>,
  blueprints: Res<Assets<WorldBlueprint<T>>>,
) where
  T::ChunkId: Send + Sync,
{
  for world_cmd in cmds.read() {
    match world_cmd {
      WorldCommand::CreateWorld {
        world_blueprint,
        seed,
      } => {
        if world_mgr.current.is_some() {
          warn!("Unable to created world, a world is still loaded");
          return;
        }
        world_mgr.current = Some((asset_server.load(world_blueprint), seed.clone()));
      }
      _ => {
        warn!("command cannot be handled");
        unimplemented!()
      }
    }
  }
}

pub fn handle_asset_events<T: WorldLayout>(
  mut cmds: Commands,
  mut events: EventReader<AssetEvent<WorldBlueprint<T>>>,
  mut world_mgr: ResMut<WorldManager<T>>,
  bps: Res<Assets<WorldBlueprint<T>>>,
) {
  for event in events.read() {
    match event {
      AssetEvent::Modified { id } => {
        let Some((handle, _)) = world_mgr.current.as_ref() else {
          return;
        };

        if id != &Into::<AssetId<WorldBlueprint<T>>>::into(handle) {
          continue;
        }

        // despawn all chunks
        for (_, e) in world_mgr.loaded_chunks.drain() {
          cmds.entity(e).despawn_recursive();
        }
        // TODO: clear cache?
      }
      _ => {}
    }
  }
}

pub fn generate_chunk_layers<T: Send + Sync + WorldChunkLayerAsset<A> + 'static, A: WorldLayout>(
  q_marker: Query<&Transform, With<WorldLoadMarker>>,
  mut cmd: Commands,
  cache: Res<WorldGenerationCache<A::ChunkId>>,
  mut world_manager: ResMut<WorldManager<A>>,
  bps: Res<Assets<WorldBlueprint<A>>>,
  settings: Res<WorldSettings>,
) {
  let (bp, seed) = {
    let Some((bp_handle, current_seed)) = world_manager.current.as_ref() else {
      return;
    };
    let Some(bp) = bps.get(bp_handle) else {
      return;
    };
    (bp, current_seed.clone())
  };

  let thread_pool = AsyncComputeTaskPool::get();

  for marker in q_marker.iter() {
    // TODO: translate marker by origin
    let mut to_generate =
      bp.layout
        .get_by_lod(&marker.translation, 0, settings.generate_lod_threshold);
    let generated: HashSet<_> = cache.chunk_cache.keys().map(|k| &k.0).cloned().collect();
    let generating: HashSet<_> = world_manager
      .pending_generation
      .keys()
      .map(|k| &k.0)
      .cloned()
      .collect();

    to_generate.retain(|c| !generated.contains(c) && !generating.contains(c));

    for chunk_id in to_generate.into_iter() {
      let entity = cmd.spawn_empty().id();
      world_manager.mark_chunk_layer_as_generating::<T>(&chunk_id, entity.clone(), false);

      let seedc = seed.clone();

      let task = thread_pool.spawn(async move {
        let _data = T::generate(&seedc, &chunk_id);
        // TODO: persist to disk
        let file = "TODO";
        (chunk_id, file.to_owned(), PhantomData)
      });

      cmd
        .entity(entity)
        .insert(WorldChunkGenerationTask::<T, A::ChunkId>(task));
    }
  }
}

pub fn poll_chunk_layer_generation_tasks<T: Send + Sync + 'static, A: WorldLayout>(
  mut cmd: Commands,
  mut tasks: Query<(Entity, &mut WorldChunkGenerationTask<T, A::ChunkId>)>,
  mut world_manager: ResMut<WorldManager<A>>,
  mut cache: ResMut<WorldGenerationCache<A::ChunkId>>,
) {
  for (entity, mut task) in &mut tasks {
    if let Some((chunk_id, file_name, _)) = block_on(future::poll_once(&mut task.0)) {
      world_manager.mark_chunk_layer_as_generating::<T>(&chunk_id, entity.clone(), true);
      cache.set_chunk_cache::<T>(&chunk_id, file_name);
      cmd.entity(entity).despawn_recursive();
    }
  }
}

pub fn spawn_chunk<L: WorldLayout>(
  mut cmd: Commands,
  q_marker: Query<&Transform, With<WorldLoadMarker>>,
  mut world_manager: ResMut<WorldManager<L>>,
  mut cache: ResMut<WorldGenerationCache<L::ChunkId>>,
  settings: Res<WorldSettings>,
  bps: Res<Assets<WorldBlueprint<L>>>,
) {
  let Some((bp_handle, current_seed)) = world_manager.current.as_ref() else {
    return;
  };
  let Some(bp) = bps.get(bp_handle) else {
    return;
  };

  for marker in q_marker.iter() {
    // TODO: translate marker by origin
    let mut to_spawn =
      bp.layout
        .get_by_lod(&marker.translation, 0, settings.visibility_lod_threshold);
    let loaded: HashSet<_> = world_manager.loaded_chunks.keys().cloned().collect();

    to_spawn.retain(|c: &<L as WorldLayout>::ChunkId| !loaded.contains(c));

    for chunk_id in to_spawn.into_iter() {
      let entity = cmd
        .spawn(WorldChunkBundle {
          chunk: WorldChunk {
            chunk_id: chunk_id.clone(),
            lod: bp.layout.get_lod_from_point(&marker.translation, &chunk_id), // TODO: synchronize lod
            ..default()
          },
          ..default()
        })
        .id();

      world_manager.mark_chunk_as_loaded(&chunk_id, entity);
    }
  }
}

pub fn spawn_chunk_layers<T: WorldChunkLayerAsset<L>, L: WorldLayout>(
  mut cmd: Commands,
  asset_server: AssetServer,
  mut cache: ResMut<WorldGenerationCache<L::ChunkId>>,
  qry: Query<(Entity, &WorldChunk<L::ChunkId>), Added<WorldChunk<L::ChunkId>>>,
) where
  <L as WorldLayout>::ChunkId: Clone + PartialEq + Eq + std::hash::Hash,
{
  for (entity, chunk) in qry.iter() {
    let key = ChunkLayerId(chunk.chunk_id.clone(), TypeId::of::<T>());
    let Some(generated_asset) = cache.chunk_cache.get(&key) else {
      continue;
    };

    cmd.entity(entity).with_children(|b| {
      b.spawn(WorldChunkLayer {
        data: asset_server.load::<T>(generated_asset),
        phatom: PhantomData,
      });
    });
  }
}
