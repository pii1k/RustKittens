use bevy::prelude::*;

use super::super::components::*;
use crate::common::animation::components::*;

pub fn draw_fov(
    q_player: Single<
        (
            &Transform,
            &AnimationController<PlayerMovementState>,
            &Player,
        ),
        With<Player>,
    >,
    mut gizmos: Gizmos,
) {
    let (transform, anim_controller, player) = q_player.into_inner();

    let position = transform.translation.truncate();

    let facing_dir = Direction8::dir_to_vec2(anim_controller.direction);
    let base_angle_in_rad = facing_dir.y.atan2(facing_dir.x);

    let segments = 30;
    let half_fov_in_rad = player.fov_angle_in_rad / 2.0;

    for i in 0..segments {
        let angle_start_in_rad = base_angle_in_rad - half_fov_in_rad
            + (player.fov_angle_in_rad * i as f32 / segments as f32);
        let angle_end_in_rad = base_angle_in_rad - half_fov_in_rad
            + (player.fov_angle_in_rad * (i + 1) as f32 / segments as f32);

        let start = position + Vec2::from_angle(angle_start_in_rad) * player.fov_range;
        let end = position + Vec2::from_angle(angle_end_in_rad) * player.fov_range;

        gizmos.line_2d(position, start, Color::srgba(1.0, 1.0, 0.0, 0.3));
        gizmos.line_2d(start, end, Color::srgba(1.0, 1.0, 0.0, 0.3));
    }

    let left = position + Vec2::from_angle(base_angle_in_rad - half_fov_in_rad) * player.fov_range;
    let right = position + Vec2::from_angle(base_angle_in_rad + half_fov_in_rad) * player.fov_range;
    gizmos.line_2d(position, left, Color::srgba(1.0, 0.5, 0.0, 0.8));
    gizmos.line_2d(position, right, Color::srgba(1.0, 0.5, 0.0, 0.8));
}
