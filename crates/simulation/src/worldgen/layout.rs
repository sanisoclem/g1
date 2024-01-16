use bevy::prelude::*;
use utils::Vec2Conversions;

/// Identifies a WorldChunk
///
/// The ChunkId is composed of the x and y coordinates of the chunk,
/// with the central chunk 0,0 at the center.
#[derive(PartialEq, Hash, Eq, Clone, Default)]
pub struct ChunkId(i16, i16);

impl Into<Vec2> for ChunkId {
  fn into(self) -> Vec2 {
    Vec2::new(self.0 as f32, self.1 as f32)
  }
}
impl Into<Vec2> for &ChunkId {
  fn into(self) -> Vec2 {
    Vec2::new(self.0 as f32, self.1 as f32)
  }
}
impl From<Vec2> for ChunkId {
  fn from(value: Vec2) -> Self {
    ChunkId(value.x as i16, value.y as i16)
  }
}
impl From<&Vec2> for ChunkId {
  fn from(value: &Vec2) -> Self {
    ChunkId(value.x as i16, value.y as i16)
  }
}

pub trait WorldLayout: TypePath + Send + Sync {
  type ChunkId: Default + PartialEq + std::hash::Hash + Eq + Clone + Sync + Send;
  type ChunkSpace: Clone + Sync + Send;

  fn to_chunk_space(&self, world_space: &Vec3, chunk: Option<&Self::ChunkId>) -> Self::ChunkSpace;
  fn to_world_space(&self, chunk_space: &Self::ChunkSpace) -> Vec3;
  fn get_lod_from_point(&self, point: &Vec3, chunk_id: &Self::ChunkId) -> u16;
  /// Gets chunkIds ordered by LOD starting at `min` (inclusive)
  fn get_by_lod(&self, point: &Vec3, min: u16, max: u16) -> Vec<Self::ChunkId>;
}

#[derive(Reflect, serde::Deserialize)]
pub struct DefaultLayout {
  height: u16,
  radius: u16,
  chunk_size: u16,
  lod_size: u16,
}

impl DefaultLayout {
  fn max_extents(&self) -> Vec2 {
    // TODO: cache this
    let chunk_dim = self.chunk_size as f32 * 2.;
    let max_x = chunk_dim * (self.radius as f32 * 2. + 1.) / 2.;
    let max_y = chunk_dim * (self.height as f32 * 2. + 1.) / 2.;
    Vec2::new(max_x, max_y)
  }
  fn normalize_coords(&self, pos: &Vec2) -> Vec2 {
    let max = self.max_extents();
    pos.rem_euclid(max)
  }
  fn get_chunk_from_world(&self, pos: &Vec2) -> (ChunkId, Vec2) {
    let normalized = self.normalize_coords(pos);
    // TODO: precalculate
    let half_chunk = Vec2::new(self.chunk_size as f32, self.chunk_size as f32);
    let full_chunk = half_chunk * 2.;
    let centered = normalized + half_chunk;
    let c = centered.div_euclid(full_chunk);
    let v = centered.rem_euclid(full_chunk) - half_chunk;
    (c.into(), v)
  }

  fn translate_chunk_coords(
    &self,
    from_chunk: &ChunkId,
    coords: &Vec2,
    to_chunk: &ChunkId,
  ) -> Vec2 {
    let half_chunk = Vec2::new(self.chunk_size as f32, self.chunk_size as f32);
    let from: Vec2 = from_chunk.into();
    let to: Vec2 = to_chunk.into();
    let delta: Vec2 = from - to;
    *coords + (delta * half_chunk * 2.)
  }
}

impl WorldLayout for DefaultLayout {
  type ChunkId = ChunkId;
  type ChunkSpace = (ChunkId, Vec3);
  fn get_lod_from_point(&self, point: &Vec3, chunk_id: &Self::ChunkId) -> u16 {
    let (_, pos) = self.to_chunk_space(point, Some(chunk_id));
    (pos.x.abs().max(pos.y.abs()) as u16) / (self.chunk_size * 2 * self.lod_size)
  }
  fn to_chunk_space(&self, world_space: &Vec3, chunk: Option<&Self::ChunkId>) -> Self::ChunkSpace {
    let xz = world_space.xz();
    let (c, v) = self.get_chunk_from_world(&xz);
    let Some(origin) = chunk else {
      return (c, v.into_3d(world_space.y));
    };

    let translated = self.translate_chunk_coords(&c, &v, &origin);
    (origin.clone(), translated.into_3d(world_space.y))
  }
  fn to_world_space(&self, chunk_space: &Self::ChunkSpace) -> Vec3 {
    let pos = chunk_space.1;
    let origin: Vec2 = chunk_space.0.clone().into();
    let half_chunk = Vec2::new(self.chunk_size as f32, self.chunk_size as f32);
    (pos.xz() + (origin * half_chunk * 2.)).into_3d(chunk_space.1.y)
  }

  fn get_by_lod(&self, point: &Vec3, min: u16, max: u16) -> Vec<Self::ChunkId> {
    let (origin, _) = self.to_chunk_space(point, None);

    let mut result = Vec::new();
    let m = max as i16;

    for i in -m..=m {
      for j in -m..=m {
        if i.abs() < min as i16 && j.abs() < min as i16 {
          continue;
        }
        result.push(ChunkId(origin.0 + i, origin.1 + j));
      }
    }
    result
  }
}

// TODO: write layout tests
