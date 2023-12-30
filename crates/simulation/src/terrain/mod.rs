use bevy::{prelude::*, utils::HashMap};

#[derive(Asset, TypePath)]
pub struct OpenWorldLevel {
  pub name: String,
  pub rebase_threshold: f32,
  pub secret_rooms: HashMap<String, String>,
  pub areas: HashMap<String, String>,
  pub assets: Option<OpenWorldLevelAssets>,
}

pub struct OpenWorldLevelAssets {
  pub secret_rooms: HashMap<String, Handle<SecretRoom>>,
  pub areas: HashMap<String, Handle<Area>>,
}

#[derive(Asset, TypePath)]
pub struct SecretRoom {
  pub name: String,
}

#[derive(Asset, TypePath)]
pub struct Area {
  pub name: String,
}

