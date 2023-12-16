use std::{collections::HashMap, sync::Arc};

use bevy::{
  asset::{io::Reader, AssetLoader, LoadContext},
  prelude::*,
  utils::BoxedFuture,
};
use rodio::{source::from_iter, Source};

use crate::processed::ProcessedAudioSource;

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub struct AudioSampleId(u32);

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub struct AudioThemeId(u32);

#[derive(Clone, Copy)]
pub struct PlaylistId(u32);

#[derive(Asset, TypePath, Clone)]
pub struct Playlist {
  id: PlaylistId,
  name: Arc<&'static str>,
  music: HashMap<AudioSampleId, Handle<AudioSource>>,
  themes: HashMap<AudioThemeId, AudioTheme>,
}

#[derive(Default)]
pub struct PlaylistLoader;

impl AssetLoader for PlaylistLoader {
  type Asset = Playlist;
  type Settings = ();
  type Error = std::io::Error;

  fn load<'a>(
    &'a self,
    reader: &'a mut Reader,
    _settings: &'a Self::Settings,
    _load_context: &'a mut LoadContext,
  ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
    unimplemented!()
  }

  fn extensions(&self) -> &[&str] {
    &["playlist"]
  }
}

#[derive(Clone)]
pub struct AudioTheme {
  pub name: Arc<&'static str>,
  pub flow: AudioFlow,
}

#[derive(Clone)]
pub enum AudioFlow {
  Loop(AudioSampleId),
  IntroLoop(AudioSampleId, AudioSampleId),
}

impl Playlist {
  pub fn get_theme(&self, theme_id: &AudioThemeId) -> Option<&AudioTheme> {
    self.themes.get(theme_id)
  }
  pub fn get_samples<'a>(
    &self,
    audio: &'a Assets<AudioSource>,
  ) -> HashMap<&'a AudioSampleId, Option<&'a AudioSource>> {
    self
      .samples
      .iter()
      .map(|(id, handle)| (id, audio.get(handle)))
      .collect()
  }

  pub fn get_audio<'a>(
    &self,
    audio: &'a Assets<AudioSource>,
  ) -> HashMap<&'a AudioSampleId, Option<&'a AudioSource>> {
    self
      .samples
      .iter()
      .map(|(id, handle)| (id, audio.get(handle)))
      .collect()
  }

  pub fn create_audio(
    &self,
    theme: &AudioFlow,
    audio: &Assets<AudioSource>,
  ) -> impl Iterator<Item = (ProcessedAudioSource)> {
    match theme {
      AudioFlow::IntroLoop(intro, main) => {
        let intro_sample = self.samples.get(intro).expect("theme to exist in playlist");
        let main_sample = self.samples.get(main).expect("theme to exist in playlist");
        let intro_source = audio.get(intro_sample).expect("sample to  be loaded");
        let main_source = audio.get(main_sample).expect("sample to be loaded");
        ProcessedAudioSource {
          sources: vec![intro_source.clone(), main_source.clone()],
          process: |sources| {
            let copy: Vec<_> = sources
              .iter()
              .enumerate()
              .map(|(i, f)| -> Box<dyn Source<Item = i16> + Send + Sync> {
                if i == 1 {
                  Box::new(f.decoder().repeat_infinite())
                } else {
                  Box::new(f.decoder())
                }
              })
              .collect();
            Box::new(from_iter(copy.into_iter()))
          },
        }
      }
      AudioFlow::Loop(main) => {
        let main_sample = self.samples.get(main).expect("theme to exist in playlist");
        let main_source = audio.get(main_sample).expect("sample to be loaded");
        ProcessedAudioSource {
          sources: vec![main_source.clone()],
          process: |sources| {
            Box::new(
              sources
                .first()
                .expect("at least one sample")
                .decoder()
                .repeat_infinite(),
            )
          },
        }
      }
    }
  }
}
