use bevy::{audio::AddAudioSource, prelude::*};

mod effects;
mod jukebox;
mod processed;
mod playlist;
mod music;

pub use effects::AudioEffect;
pub use processed::ProcessedAudio;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_source::<processed::ProcessedAudioSource>()
            .init_resource::<jukebox::Jukebox>()
            .add_event::<jukebox::JukeboxCommand>()
            .add_systems(
                Update,
                (
                    effects::apply_audio_effects,
                    (
                        jukebox::wait_for_jukebox_init,
                        jukebox::process_music_commands,
                    )
                        .chain(),
                ),
            );
    }
}
