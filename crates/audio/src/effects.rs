use bevy::prelude::*;
use std::time::Duration;
use utils::lerp;

#[derive(Component)]
pub enum AudioEffect {
  FadeTo(FadeTo),
}

#[derive(Default)]
pub struct FadeTo {
  pub target: Option<f32>,
  pub fade_length: Duration,
  pub despawn_on_zero: bool,
  pub linger: Timer,
}

impl AudioEffect {
  pub fn fade_in(duration: Duration) -> Self {
    AudioEffect::FadeTo(FadeTo {
      target: Some(1.0),
      fade_length: duration,
      despawn_on_zero: false,
      linger: Timer::from_seconds(3.0, TimerMode::Once),
    })
  }
  pub fn fade_out_despawn(duration: Duration) -> Self {
    AudioEffect::FadeTo(FadeTo {
      target: Some(0.0),
      fade_length: duration,
      despawn_on_zero: true,
      linger: Timer::from_seconds(3.0, TimerMode::Once),
    })
  }
}

pub fn apply_audio_effects(
  mut cmd: Commands,
  mut qry: Query<(Entity, &AudioSink, &mut AudioEffect)>,
  dt: Res<Time>,
) {
  for (e, sink, mut effect) in qry.iter_mut() {
    let AudioEffect::FadeTo(fade) = effect.as_mut();
    if let Some(target) = fade.target {
      let new_volume = lerp(
        sink.volume(),
        target,
        dt.delta_seconds() / fade.fade_length.as_secs_f32(),
      );
      sink.set_volume(new_volume);
      let target_reached = (new_volume - target).abs() <= 0.001;

      if target_reached {
        info!("target reached {:?}", e);
        fade.target = None;
      }
    }

    if sink.volume() <= 0.001 {
      if fade.despawn_on_zero {
        fade.linger.tick(dt.delta());
        if fade.linger.just_finished() {
          info!("depawned {:?}", e);
          cmd.entity(e).despawn();
        }
      } else if !sink.is_paused() {
        sink.pause();
      }
    }
    if sink.is_paused() && sink.volume() > 0.001 {
      sink.play()
    }
  }
}
