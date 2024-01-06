// use bevy::{
//   pbr::{MaterialPipeline, MaterialPipelineKey},
//   prelude::*,
//   render::{
//     mesh::{MeshVertexAttribute, MeshVertexBufferLayout},
//     render_resource::{
//       RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError, VertexFormat, AsBindGroup,
//     },
//   },
// };

// use crate::worldgen::{WorldChunkLayerAsset, ChunkId, WorldSeed};

// const ATTRIBUTE_BLEND_COLOR: MeshVertexAttribute =
//   MeshVertexAttribute::new("BlendColor", 988540917, VertexFormat::Float32x4);

// #[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
// pub struct TerrainMaterial {
//   #[uniform(0)]
//   color: Color,
// }

// impl Material for TerrainMaterial {
//   fn vertex_shader() -> ShaderRef {
//     "shaders/terrain.wgsl".into()
//   }
//   fn fragment_shader() -> ShaderRef {
//     "shaders/terrain.wgsl".into()
//   }

//   fn specialize(
//     _pipeline: &MaterialPipeline<Self>,
//     descriptor: &mut RenderPipelineDescriptor,
//     layout: &MeshVertexBufferLayout,
//     _key: MaterialPipelineKey<Self>,
//   ) -> Result<(), SpecializedMeshPipelineError> {
//     let vertex_layout = layout.get_layout(&[
//       Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
//       ATTRIBUTE_BLEND_COLOR.at_shader_location(1),
//     ])?;
//     descriptor.vertex.buffers = vec![vertex_layout];
//     Ok(())
//   }
// }

// #[derive(Asset, TypePath)]
// pub struct TerrainChunk {}
// pub struct TerrainChunkData {
//   mesh: Mesh, // TODO: lod meshes
//   material: TerrainMaterial,
// }

// impl WorldChunkLayerAsset for TerrainChunk {
//   type PersistAs = TerrainChunkData;
//   fn generate(seed: &WorldSeed, chunk_id: &ChunkId) -> Self::PersistAs {

//   }
// }
