use std::{collections::HashMap, sync::Arc, time::Duration};

use bevy::prelude::*;

use crate::{
  playlist::{AudioTheme, AudioThemeId, Playlist, PlaylistId},
  processed::ProcessedAudioSource,
  AudioEffect,
};

#[derive(Resource, Default)]
pub struct Jukebox {
  now_playing: Option<AudioThemeId>,
  loaded_themes: HashMap<AudioThemeId, JukeboxTheme>,
  loaded_paylists: HashMap<PlaylistId, JukeboxPlaylist>,
  loading: Vec<Handle<Playlist>>,
}

pub struct JukeboxTheme {
  name: Arc<&'static str>,
  audio: Handle<ProcessedAudioSource>,
}

pub struct JukeboxPlaylist {
  name: Arc<&'static str>,
  themes: Vec<AudioThemeId>,
}

impl Jukebox {
  pub fn add_playlist(&mut self, playlist_handle: Handle<Playlist>) {
    self.loading.push(playlist_handle);
  }
  // pub(crate) fn play_theme(
  //   &self,
  //   theme_id: &AudioThemeId,
  //   playlists: &Assets<Playlist>,
  //   audio: &Assets<AudioSource>,
  //   processed_audio: &Assets<ProcessedAudioSource>,
  // ) -> Option<(AudioThemePlayer,Handle<ProcessedAudioSource>) > {
  //   for (_, playlist_handle) in self.playlists {
  //     let Some(playlist) = playlists.get(playlist_handle) else {
  //       continue;
  //     };
  //     let Some(theme)  = playlist.get_theme(theme_id) else {
  //       continue;
  //     };

  //     let audio_source = playlist.create_audio(&theme.flow, audio);
  //     return
  //   }
  //   None
  // }
}

#[derive(Event)]
pub enum JukeboxCommand {
  PlayTheme(AudioThemeId),
}

#[derive(Component)]
pub struct AudioThemePlayer {
  pub id: AudioThemeId,
  pub name: Arc<&'static str>,
}

pub fn process_music_commands(
  qry: Query<(Entity, &AudioThemePlayer)>,
  mut commands: Commands,
  mut cmds: EventReader<JukeboxCommand>,
  jukebox: ResMut<Jukebox>,
  audio_sources: Res<Assets<AudioSource>>,
  playlists: Res<Assets<Playlist>>,
) {
  let mut started = false;
  for cmd in cmds.read() {
    if started {
      break;
    }
    let JukeboxCommand::PlayTheme(theme_id) = cmd;

    // look for currently playing music that matches
    for (e, b) in qry.iter() {
      if &b.id == theme_id {
        commands
          .entity(e)
          .insert(AudioEffect::fade_in(Duration::from_secs_f32(1.0)));
        started = true;
        continue;
      }
      commands
        .entity(e)
        .insert(AudioEffect::fade_out_despawn(Duration::from_secs_f32(0.2)));
    }
    if !started {
      let Some(theme) = jukebox.loaded_themes.get(&theme_id) else {
        continue;
      };

      commands.spawn((AudioSourceBundle {
        source: theme.audio.clone(),
        settings: PlaybackSettings::LOOP,
      },));
      started = true;
    }
  }
}

pub fn process_loaded_playlists(
  mut jukebox: ResMut<Jukebox>,
  audio: Res<Assets<AudioSource>>,
  playlists: Res<Assets<Playlist>>,
  processed_audio: ResMut<Assets<ProcessedAudioSource>>,
) {
  for phandle in jukebox.loading.iter() {
    let Some(playlist) = playlists.get(phandle) else {
      continue;
    };
    let samples = playlist.get_samples(&audio);
    if samples.
  }
}
