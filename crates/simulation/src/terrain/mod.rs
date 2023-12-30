use bevy::{prelude::*, utils::HashMap};

/// what is this defining?
///  - world seed - how can we regenerate this exact same world
///  - destinations - what are the key destinations and how are they connected
///  -
///
/// Notes:
///  - all operations should be deterministic so we can cache all output
///  - the player may change the world so we need to implement some CoW mechanism to save changes
#[derive(Asset, TypePath)]
pub struct CylinderWorldBlueprint {
  pub name: String,
  pub height: u16,
  pub radius: u16,
  pub seed: [u8; 16],
  pub areas: HashMap<String, String>,
  pub rooms: HashMap<String, String>,
  pub assets: Option<WorldAssets>,
}

pub struct WorldAssets {
  pub rooms: HashMap<String, Handle<SecretRoom>>,
  pub areas: HashMap<String, Handle<Area>>,
}

#[derive(Asset, TypePath)]
pub struct Room {
  pub name: String,
}

#[derive(Asset, TypePath)]
pub struct Area {
  pub name: String,
}
