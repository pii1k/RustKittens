use std::time::Duration;

use bevy::prelude::*;

use super::components::*;

pub fn handle_animation_state_change<S: AnimationState>(mut query: AnimationChangedQuery<S>) {
    for (anim_set, mut anim_controller, mut sprite) in &mut query {
        let clip_name = anim_controller.state.clip_name();
        let Some(clip) = anim_set.get_clip(clip_name) else {
            warn!("Animation clip '{}' not found", clip_name);
            continue;
        };

        if sprite.image != clip.image_handle {
            sprite.image = clip.image_handle.clone();

            let row = anim_controller.direction as usize;
            let initial_idx = row * clip.frames_per_direction;

            sprite.texture_atlas = Some(TextureAtlas {
                layout: clip.texture_layout_handle.clone(),
                index: initial_idx,
            });

            anim_controller.current_frame_idx = 0;
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
            let next_frame = anim_controller.current_frame_idx + 1;

            if !clip.looping && next_frame >= clip.frames_per_direction {
                anim_controller.current_frame_idx = clip.frames_per_direction - 1;
                anim_controller.is_finished = true;
            } else if clip.looping {
                anim_controller.current_frame_idx =
                    (anim_controller.current_frame_idx + 1) % clip.frames_per_direction;
                anim_controller.is_finished = false;
            } else {
                anim_controller.current_frame_idx = next_frame;
            }

            let row = anim_controller.direction as usize;
            let idx = row * clip.frames_per_direction + anim_controller.current_frame_idx;

            if let Some(ref mut atlas) = sprite.texture_atlas {
                atlas.index = idx;
            }
        }
    }
}

pub fn auto_return_upper_body_to_normal(
    mut q_anim_controller: Query<&mut AnimationController<AnimationLayers>>,
) {
    for mut anim_controller in &mut q_anim_controller {
        if anim_controller.state.upper_body.is_non_looping()
            && anim_controller.is_animation_finished()
        {
            let mut new_layers = anim_controller.state.clone();
            new_layers.upper_body = UpperBodyState::Normal;
            anim_controller.change_state(new_layers);
        }
    }
}
