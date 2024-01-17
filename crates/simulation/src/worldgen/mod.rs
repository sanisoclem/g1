use assets::RonAssetApp;
use bevy::prelude::*;

use self::{
  resource::{WorldGenerationCache, WorldManager, WorldSettings},
  system::{
    generate_chunk_layers, handle_asset_events, handle_world_commands,
    poll_chunk_layer_generation_tasks, spawn_chunk, spawn_chunk_layers,
  },
  terrain::{TerrainChunk, spawn_mesh},
};

pub use asset::{WorldBlueprint, WorldChunkLayerAsset};
pub use component::*;
pub use layout::{ChunkId, DefaultLayout, WorldLayout};

mod asset;
mod component;
mod layout;
mod resource;
mod system;
mod terrain;

pub trait WorldGenApp {
  fn add_default_world_gen(&mut self) -> &mut Self;
  fn add_worldgen<T: WorldLayout>(&mut self) -> &mut Self;
  fn register_chunk_layer<A, T: WorldLayout>(&mut self) -> &mut Self
  where
    A: WorldChunkLayerAsset<T> + Send + Sync + 'static;
}

impl WorldGenApp for App {
  fn add_default_world_gen(&mut self) -> &mut Self {
    self
      .add_worldgen::<DefaultLayout>()
      .register_chunk_layer::<terrain::TerrainChunk, DefaultLayout>()
      .register_ron_asset::<TerrainChunk>()
      // .register_type::<WorldChunkLayer<TerrainChunk,DefaultLayout>>()
      .add_systems(Update, spawn_mesh)
  }
  fn add_worldgen<T: WorldLayout>(&mut self) -> &mut Self {
    self
      .init_resource::<WorldSettings>()
      .init_resource::<WorldManager<T>>()
      .init_resource::<WorldGenerationCache<T::ChunkId>>()
      .add_event::<WorldCommand>()
      // .register_type::<WorldChunk<T>>()
      .register_ron_asset::<WorldBlueprint<DefaultLayout>>()
      .add_systems(
        Update,
        (
          handle_world_commands::<T>,
          handle_asset_events::<T>,
          spawn_chunk::<T>,
        ),
      )
  }
  fn register_chunk_layer<A, T: WorldLayout>(&mut self) -> &mut Self
  where
    A: WorldChunkLayerAsset<T> + Send + Sync + 'static,
  {
    self.add_systems(
      Update,
      (
        generate_chunk_layers::<A, T>,
        poll_chunk_layer_generation_tasks::<A, T>,
        spawn_chunk_layers::<A, T>,
      ),
    )
  }
}

/// The public command API.
///
/// Thie module is designed with the expectation that the consumer
/// will interact with it by sending commands and/or reacting to state changes.
#[derive(Event)]
pub enum WorldCommand {
  CreateWorld {
    world_blueprint: String,
    seed: WorldSeed,
  },
  LoadWorld {
    world_blueprint: String,
    world_state: String,
  },
  GoToRoom(RoomId),
  LeaveRoom,
  Unload,
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum WorldState {
  #[default]
  Disabled,
  Loading,
  InWorld,
}

#[derive(PartialEq, Hash, Eq, Clone, Default)]
pub struct RoomId(u16);

/// The seed data used to generate the world
#[derive(PartialEq, Hash, Eq, Clone, Default,Debug)]
pub struct WorldSeed([u8; 16]);

impl std::fmt::Display for WorldSeed {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(""))
  }
}