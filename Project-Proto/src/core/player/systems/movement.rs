use bevy::prelude::*;

use crate::{
    common::animation::components::AnimationController,
    core::player::components::{Player, PlayerMovementState},
};

const PLAYER_SPEED: f32 = 150.0;

pub fn move_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    q_player: Single<
        (
            &mut Player,
            &mut Transform,
            &mut AnimationController<PlayerMovementState>,
        ),
        With<Player>,
    >,
) {
    let mut direction = Vec2::new(0.0, 0.0);

    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    let (mut player, mut player_transform, mut anim_controller) = q_player.into_inner();

    player.velocity = direction.normalize_or_zero();

    if direction != Vec2::ZERO {
        direction = direction.normalize();
        direction.y /= 2.0;

        let movement = direction * PLAYER_SPEED * time.delta_secs();
        player_transform.translation += movement.extend(0.0);
    } else {
        anim_controller.current_frame_idx = 0;
    }
}
