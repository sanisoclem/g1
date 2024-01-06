use assets::RonAsset;
use bevy::{
  asset::LoadContext,
  pbr::{MaterialPipeline, MaterialPipelineKey},
  prelude::*,
  render::{
    mesh::{MeshVertexAttribute, MeshVertexBufferLayout},
    render_resource::{
      AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError, VertexFormat,
    },
  },
};

use crate::worldgen::{ChunkId, DefaultLayout, WorldChunkLayer, WorldChunkLayerAsset};

const ATTRIBUTE_BLEND_COLOR: MeshVertexAttribute =
  MeshVertexAttribute::new("BlendColor", 988540917, VertexFormat::Float32x4);

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TerrainMaterial {
  #[uniform(0)]
  color: Color,
}

impl Material for TerrainMaterial {
  fn vertex_shader() -> ShaderRef {
    "shaders/terrain.wgsl".into()
  }
  fn fragment_shader() -> ShaderRef {
    "shaders/terrain.wgsl".into()
  }

  fn specialize(
    _pipeline: &MaterialPipeline<Self>,
    descriptor: &mut RenderPipelineDescriptor,
    layout: &MeshVertexBufferLayout,
    _key: MaterialPipelineKey<Self>,
  ) -> Result<(), SpecializedMeshPipelineError> {
    let vertex_layout = layout.get_layout(&[
      Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
      ATTRIBUTE_BLEND_COLOR.at_shader_location(1),
    ])?;
    descriptor.vertex.buffers = vec![vertex_layout];
    Ok(())
  }
}

#[derive(Asset, TypePath)]
pub struct TerrainChunk {
  mesh: String,
  material: String,
  assets: TerrainChunkNestedAssets,
}

pub struct TerrainChunkNestedAssets {
  mesh: Handle<Mesh>,
  material: Handle<StandardMaterial>,
}

impl RonAsset for TerrainChunk {
  type NestedAssets = TerrainChunkNestedAssets;
  fn construct_nested_assets<'a>(&mut self, load_context: &'a mut LoadContext) {
    self.assets = TerrainChunkNestedAssets {
      mesh: load_context.load(&self.mesh),
      material: load_context.load(&self.material),
    };
  }
  fn extensions() -> &'static [&'static str] {
    &["terrain.chunk.ron"]
  }
}

pub struct TerrainChunkData {
  mesh: Mesh, // TODO: lod meshes
  material: TerrainMaterial,
}

impl WorldChunkLayerAsset<DefaultLayout> for TerrainChunk {
  type PersistAs = TerrainChunkData;
  fn generate(seed: &crate::worldgen::WorldSeed, chunk_id: &ChunkId) -> Self::PersistAs {
    // TODO: generate mesh
    unimplemented!()
  }
}

pub fn spawn_mesh(
  mut cmd: Commands,
  qry: Query<
    (Entity, &WorldChunkLayer<TerrainChunk, DefaultLayout>),
    Added<WorldChunkLayer<TerrainChunk, DefaultLayout>>,
  >,
  terrain_assets: Res<Assets<TerrainChunk>>,
) {
  for (entity, tc) in qry.iter() {
    let Some(d) = terrain_assets.get(&tc.data) else {
      warn!("Unable to spawn mesh, terrain asset not found");
      continue;
    };

    cmd.entity(entity).insert(PbrBundle {
      mesh: d.assets.mesh.clone(),
      material: d.assets.material.clone(),
      ..default()
    });
  }
}
