use std::time::Duration;

use bevy::prelude::*;

use crate::common::animation::components::{AnimationController, AnimationSet, AnimationState};

pub fn handle_animation_state_change<S: AnimationState>(
    mut query: Query<
        (&AnimationSet, &mut AnimationController<S>, &mut Sprite),
        Changed<AnimationController<S>>,
    >,
) {
    for (anim_set, mut anim_controller, mut sprite) in &mut query {
        let clip_name = anim_controller.state.clip_name();
        let Some(clip) = anim_set.get_clip(clip_name) else {
            warn!("Animation clip '{}' not found", clip_name);
            continue;
        };

        if sprite.image != clip.image_handle {
            sprite.image = clip.image_handle.clone();
            sprite.texture_atlas = Some(TextureAtlas {
                layout: clip.texture_layout_handle.clone(),
                index: 0,
            });

            anim_controller.current_frame = 0;
            anim_controller.frame_timer.reset();
        }
    }
}

pub fn animate_sprites<S: AnimationState>(
    time: Res<Time>,
    mut query: Query<(&AnimationSet, &mut AnimationController<S>, &mut Sprite)>,
) {
    for (anim_set, mut anim_controller, mut sprite) in &mut query {
        let clip_name = anim_controller.state.clip_name();
        let Some(clip) = anim_set.get_clip(clip_name) else {
            continue;
        };

        anim_controller
            .frame_timer
            .set_duration(Duration::from_secs_f32(clip.frame_duration));
        anim_controller.frame_timer.tick(time.delta());

        if anim_controller.frame_timer.just_finished() {
            if clip.looping {
                anim_controller.current_frame =
                    (anim_controller.current_frame + 1) % clip.frames_per_direction;
            } else {
                anim_controller.current_frame =
                    (anim_controller.current_frame + 1).min(clip.frames_per_direction - 1);
            }

            let row = anim_controller.direction as usize;
            let idx = row * clip.frames_per_direction + anim_controller.current_frame;

            if let Some(ref mut atlas) = sprite.texture_atlas {
                atlas.index = idx;
            }
        }
    }
}
