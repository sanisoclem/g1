use assets::RonAssetApp;
use bevy::prelude::*;

use self::{
  asset::{WorldBlueprint, WorldChunkLayerAsset},
  layout::{DefaultLayout, WorldLayout},
  system::{
    generate_chunk_layers, handle_asset_events, handle_world_commands,
    poll_chunk_layer_generation_tasks, spawn_chunk, spawn_chunk_layers,
  },
};

mod asset;
mod component;
mod layout;
mod resource;
mod system;

pub trait WorldGenApp {
  fn add_layout<T: WorldLayout>(&mut self) -> &mut Self;
  fn register_chunk_layer<A, T: WorldLayout>(&mut self) -> &mut Self
  where
    A: WorldChunkLayerAsset<T> + Send + Sync + 'static;
}

impl WorldGenApp for App {
  fn add_layout<T: WorldLayout>(&mut self) -> &mut Self {
    self
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
#[derive(PartialEq, Hash, Eq, Clone, Default)]
pub struct WorldSeed([u8; 16]);
