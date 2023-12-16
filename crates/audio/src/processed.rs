use bevy::{audio::Source, prelude::*};

#[derive(Asset, TypePath, Clone)]
pub struct ProcessedAudioSource {
  pub sources: Vec<AudioSource>,
  pub process: fn(sources: &Vec<AudioSource>) -> Box<dyn Source<Item = i16> + Sync + Send>,
}

impl Decodable for ProcessedAudioSource {
  type DecoderItem = i16;
  type Decoder = Box<dyn Source<Item = i16> + Sync + Send>;
  fn decoder(&self) -> Self::Decoder {
    (self.process)(&self.sources)
  }
}

#[derive(Component)]
pub struct ProcessedAudio {
  pub sources: Vec<Handle<AudioSource>>,
  pub process: fn(sources: &Vec<AudioSource>) -> Box<dyn Source<Item = i16> + Sync + Send>,
}
