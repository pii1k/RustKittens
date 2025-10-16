use bevy::prelude::*;

use crate::{
    common::animation::components::{AnimationController, Direction8},
    core::player::components::{Player, PlayerState},
};

const CURSOR_DISPLACEMENT: f32 = 5.0;

pub fn aim_at_cursor(
    q_window: Query<&Window, With<Window>>,
    q_camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    q_player: Single<(&Transform, &mut AnimationController<PlayerState>), With<Player>>,
) {
    let window = q_window.single();
    let (camera, camera_transform) = *q_camera;

    if let Some(cursor_pos) = window.cursor_position()
        && let Ok(cursor_world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos)
    {
        let (transform, mut anim_controller) = q_player.into_inner();

        let player_pos = transform.translation.truncate();
        let dir_to_cursor = cursor_world_pos - player_pos;

        if dir_to_cursor.length() > CURSOR_DISPLACEMENT {
            anim_controller.direction = Direction8::from_velocity(dir_to_cursor);
        }
    }
}

pub fn shoot(mouse_input: Res<ButtonInput<MouseButton>>) {
    if mouse_input.just_pressed(MouseButton::Left) {
        println!("Shot's fired!!");
    }
}
