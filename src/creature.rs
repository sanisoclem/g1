use bevy::prelude::*;

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct CreatureBlueprint {
  pub mesh: Handle<Mesh>,
  pub animation_set: String,
  pub stationary: bool,
}

#[derive(Resource)]
pub struct CreatureLibrary