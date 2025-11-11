use bevy::prelude::*;

use super::super::components::*;
use crate::common::animation::components::*;

const PLAYER_SPEED: f32 = 150.0;

pub fn move_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    q_player: Single<
        (
            &mut Player,
            &mut Transform,
            &mut AnimationController<AnimationLayers>,
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

    let is_moving = direction.length() > 0.0;

    if is_moving {
        direction = direction.normalize();

        anim_controller.direction = Direction8::from_velocity(player.velocity);

        direction.y /= 2.0;

        let movement = direction * PLAYER_SPEED * time.delta_secs();
        player_transform.translation += movement.extend(0.0);
    }

    update_lower_body_state(&mut anim_controller, is_moving);
}

fn update_lower_body_state(
    anim_controller: &mut AnimationController<AnimationLayers>,
    is_moving: bool,
) {
    let new_lower = if is_moving {
        LowerBodyState::Walk
    } else {
        LowerBodyState::Idle
    };

    if anim_controller.state.lower_body != new_lower {
        let mut new_layers = anim_controller.state.clone();
        new_layers.lower_body = new_lower;
        anim_controller.change_state(new_layers);
    }
}
