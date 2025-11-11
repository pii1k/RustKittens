use bevy::prelude::*;

use super::super::components::*;
use crate::common::animation::components::*;

const CURSOR_DISPLACEMENT: f32 = 5.0;

pub fn aim_at_cursor(
    q_window: Query<&Window, With<Window>>,
    q_camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    q_player: Single<
        (
            &Transform,
            &mut Player,
            &mut AnimationController<AnimationLayers>,
        ),
        With<Player>,
    >,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    let (transform, mut player, mut anim_controller) = q_player.into_inner();

    player.is_aiming = mouse_input.pressed(MouseButton::Right);
    if !player.is_aiming {
        return;
    }

    let window = q_window.single();
    let (camera, camera_transform) = *q_camera;

    if let Some(cursor_pos) = window.cursor_position()
        && let Ok(cursor_world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos)
    {
        let player_pos = transform.translation.truncate();
        let dir_to_cursor = cursor_world_pos - player_pos;

        if dir_to_cursor.length() > CURSOR_DISPLACEMENT {
            anim_controller.direction = Direction8::from_velocity(dir_to_cursor);
        }
    }
}

pub fn shoot(
    mouse_input: Res<ButtonInput<MouseButton>>,
    q_player: Single<(&Player, &mut AnimationController<AnimationLayers>), With<Player>>,
) {
    let (player, mut anim_controller) = q_player.into_inner();

    if anim_controller.state.upper_body != UpperBodyState::Normal {
        return;
    }

    if !player.is_aiming {
        return;
    }

    if mouse_input.just_pressed(MouseButton::Left) {
        let mut new_layers = anim_controller.state.clone();
        new_layers.upper_body = UpperBodyState::Attack;
        anim_controller.change_state(new_layers);
    }
}
