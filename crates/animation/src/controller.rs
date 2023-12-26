use bevy::prelude::*;
use std::ops::Deref;

use crate::{AnimationControllerData, AnimationControllerInput, Animator, AnimatorTarget};

pub trait AnimationController: Asset + Send {
  type ControllerData: Send + Sync;
  fn update_animation(
    &self,
    parameters: &AnimationControllerInput,
    trigger: Option<&str>,
    data: &mut Self::ControllerData,
    player: &mut AnimationPlayer,
  );
}

pub fn find_rig_target<T: AnimationController>(
  mut cmd: Commands,
  mut qry: Query<
    (Entity, &Animator<T>),
    Or<(
      Changed<Children>,
      Without<AnimatorTarget>,
      Changed<Animator<T>>,
    )>,
  >,
  mut targets: Query<&mut AnimatorTarget>,
  children: Query<&Children>,
  names: Query<&Name>,
) {
  for (e, animator) in qry.iter_mut() {
    let rig = if let Some(path) = &animator.rig_path {
      let mut cache = Vec::new();

      entity_from_path2(e, path, &children, &names, &mut cache)
    } else {
      None
    };

    if let Ok(mut target) = targets.get_mut(e) {
      target.rig_target = rig;
    } else {
      cmd.entity(e).insert(AnimatorTarget { rig_target: rig });
    }
  }
}

pub fn play_animations<T: AnimationController + Asset>(
  mut controllers: ResMut<Assets<T>>,
  mut qry: Query<
    (
      &AnimatorTarget,
      &Animator<T>,
      &AnimationControllerInput,
      &mut AnimationControllerData<T>,
    ),
    Changed<AnimationControllerInput>,
  >,
  mut qry_player: Query<&mut AnimationPlayer>,
) {
  for (target, animator, params, mut data) in qry.iter_mut() {
    let Some(rig_target) = target.rig_target else {
      continue;
    };
    let Ok(mut player) = qry_player.get_mut(rig_target) else {
      continue;
    };
    let Some(controller) = controllers.get_mut(animator.controller.clone()) else {
      continue;
    };

    controller.update_animation(params, None, &mut data.data, &mut player);
  }
}

fn entity_from_path2(
  root: Entity,
  path: &EntityPath,
  children: &Query<&Children>,
  names: &Query<&Name>,
  path_cache: &mut Vec<Option<Entity>>,
) -> Option<Entity> {
  // PERF: finding the target entity can be optimised
  let mut current_entity = root;
  path_cache.resize(path.parts.len(), None);

  let parts = path.parts.iter().enumerate();

  for (idx, part) in parts {
    let mut found = false;
    let Ok(children) = children.get(current_entity) else {
      warn!(
        "Cannot find rig target {:?}. no children found for {:?}",
        path, current_entity
      );
      return None;
    };
    if let Some(cached) = path_cache[idx] {
      if children.contains(&cached) {
        if let Ok(name) = names.get(cached) {
          if name == part {
            current_entity = cached;
            found = true;
          }
        }
      }
    }
    if !found {
      for child in children.deref() {
        let name = names.get(*child).unwrap_or(part);

        if name == part {
          current_entity = *child;
          path_cache[idx] = Some(*child);
          found = true;
          break;
        }
      }
    }
    if !found {
      warn!("Entity not found for path {:?} on part {:?}", path, part);
      return None;
    }
  }
  Some(current_entity)
}
