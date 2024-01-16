use assets::RonAsset;
use bevy::{
  asset::LoadContext,
  pbr::{MaterialPipeline, MaterialPipelineKey},
  prelude::*,
  render::{
    mesh::{Indices, MeshVertexAttribute, MeshVertexBufferLayout},
    render_resource::{
      AsBindGroup, PrimitiveTopology, RenderPipelineDescriptor, ShaderRef,
      SpecializedMeshPipelineError, VertexFormat,
    },
  },
};
use serde::Deserialize;

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

#[derive(Deserialize, Asset, Reflect)]
pub struct TerrainChunk {
  mesh: String,
  material: String,
  #[serde(skip_deserializing)]
  assets: Option<TerrainChunkNestedAssets>,
}

#[derive(Reflect)]
pub struct TerrainChunkNestedAssets {
  mesh: Handle<Mesh>,
  material: Handle<StandardMaterial>,
}

impl RonAsset for TerrainChunk {
  type NestedAssets = TerrainChunkNestedAssets;
  fn construct_nested_assets<'a>(&mut self, load_context: &'a mut LoadContext) {
    self.assets = Some(TerrainChunkNestedAssets {
      mesh: load_context.load(&self.mesh),
      material: load_context.load(&self.material),
    });
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
    TerrainChunkData {
      material: TerrainMaterial { color: Color::RED },
      mesh: create_mesh(),
    }
  }
}

fn create_mesh() -> Mesh {
  Mesh::new(PrimitiveTopology::TriangleList)
    .with_inserted_attribute(
      Mesh::ATTRIBUTE_POSITION,
      // Each array is an [x, y, z] coordinate in local space.
      // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
      // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
      vec![
        // top (facing towards +y)
        [-0.5, 0.5, -0.5], // vertex with index 0
        [0.5, 0.5, -0.5],  // vertex with index 1
        [0.5, 0.5, 0.5],   // etc. until 23
        [-0.5, 0.5, 0.5],
        // bottom   (-y)
        [-0.5, -0.5, -0.5],
        [0.5, -0.5, -0.5],
        [0.5, -0.5, 0.5],
        [-0.5, -0.5, 0.5],
        // right    (+x)
        [0.5, -0.5, -0.5],
        [0.5, -0.5, 0.5],
        [0.5, 0.5, 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
        [0.5, 0.5, -0.5],
        // left     (-x)
        [-0.5, -0.5, -0.5],
        [-0.5, -0.5, 0.5],
        [-0.5, 0.5, 0.5],
        [-0.5, 0.5, -0.5],
        // back     (+z)
        [-0.5, -0.5, 0.5],
        [-0.5, 0.5, 0.5],
        [0.5, 0.5, 0.5],
        [0.5, -0.5, 0.5],
        // forward  (-z)
        [-0.5, -0.5, -0.5],
        [-0.5, 0.5, -0.5],
        [0.5, 0.5, -0.5],
        [0.5, -0.5, -0.5],
      ],
    )
    // Set-up UV coordinated to point to the upper (V < 0.5), "dirt+grass" part of the texture.
    // Take a look at the custom image (assets/textures/array_texture.png)
    // so the UV coords will make more sense
    // Note: (0.0, 0.0) = Top-Left in UV mapping, (1.0, 1.0) = Bottom-Right in UV mapping
    .with_inserted_attribute(
      Mesh::ATTRIBUTE_UV_0,
      vec![
        // Assigning the UV coords for the top side.
        [0.0, 0.2],
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 0.25],
        // Assigning the UV coords for the bottom side.
        [0.0, 0.45],
        [0.0, 0.25],
        [1.0, 0.25],
        [1.0, 0.45],
        // Assigning the UV coords for the right side.
        [1.0, 0.45],
        [0.0, 0.45],
        [0.0, 0.2],
        [1.0, 0.2],
        // Assigning the UV coords for the left side.
        [1.0, 0.45],
        [0.0, 0.45],
        [0.0, 0.2],
        [1.0, 0.2],
        // Assigning the UV coords for the back side.
        [0.0, 0.45],
        [0.0, 0.2],
        [1.0, 0.2],
        [1.0, 0.45],
        // Assigning the UV coords for the forward side.
        [0.0, 0.45],
        [0.0, 0.2],
        [1.0, 0.2],
        [1.0, 0.45],
      ],
    )
    // For meshes with flat shading, normals are orthogonal (pointing out) from the direction of
    // the surface.
    // Normals are required for correct lighting calculations.
    // Each array represents a normalized vector, which length should be equal to 1.0.
    .with_inserted_attribute(
      Mesh::ATTRIBUTE_NORMAL,
      vec![
        // Normals for the top side (towards +y)
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        // Normals for the bottom side (towards -y)
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        // Normals for the right side (towards +x)
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        // Normals for the left side (towards -x)
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        // Normals for the back side (towards +z)
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        // Normals for the forward side (towards -z)
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
      ],
    )
    // Create the triangles out of the 24 vertices we created.
    // To construct a square, we need 2 triangles, therefore 12 triangles in total.
    // To construct a triangle, we need the indices of its 3 defined vertices, adding them one
    // by one, in a counter-clockwise order (relative to the position of the viewer, the order
    // should appear counter-clockwise from the front of the triangle, in this case from outside the cube).
    // Read more about how to correctly build a mesh manually in the Bevy documentation of a Mesh,
    // further examples and the implementation of the built-in shapes.
    .with_indices(Some(Indices::U32(vec![
      0, 3, 1, 1, 3, 2, // triangles making up the top (+y) facing side.
      4, 5, 7, 5, 6, 7, // bottom (-y)
      8, 11, 9, 9, 11, 10, // right (+x)
      12, 13, 15, 13, 14, 15, // left (-x)
      16, 19, 17, 17, 19, 18, // back (+z)
      20, 21, 23, 21, 22, 23, // forward (-z)
    ])))
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
    let Some(assets) = &d.assets else {
      warn!("Unable to spawn mesh, terrain's related assets not found ");
      continue;
    };

    cmd.entity(entity).insert(PbrBundle {
      mesh: assets.mesh.clone(),
      material: assets.material.clone(),
      ..default()
    });
  }
}
