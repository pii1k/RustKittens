use bevy::prelude::*;

use crate::common::animation::components::{AnimationController, Direction8};

impl Default for AnimationController {
    fn default() -> Self {
        Self {
            current_frame: 0,
            frame_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            direction: Direction8::South,
        }
    }
}

impl Direction8 {
    pub fn from_velocity(velocity: Vec2) -> Self {
        if velocity.length() < 0.01 {
            return Direction8::South;
        }

        let angle = velocity.y.atan2(velocity.x);
        let degrees = angle.to_degrees();

        let normalized = (degrees + 22.5) / 45.0;
        let idx = ((normalized.floor() as i32 + 8) % 8) as usize;

        match idx {
            0 => Self::East,      // 0°
            1 => Self::NorthEast, // 45°
            2 => Self::North,     // 90°
            3 => Self::NorthWest, // 135°
            4 => Self::West,      // 180°
            5 => Self::SouthWest, // 225°
            6 => Self::South,     // 270°
            7 => Self::SouthEast, // 315°
            _ => Self::South,
        }
    }
}

pub fn update_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationController, &mut Sprite)>,
) {
    for (mut anim_controller, mut sprite) in &mut query {
        anim_controller.frame_timer.tick(time.delta());

        if anim_controller.frame_timer.just_finished() {
            anim_controller.current_frame = (anim_controller.current_frame + 1) % 8;

            let row = anim_controller.direction as usize;
            let col = anim_controller.current_frame;

            if let Some(ref mut atlas) = sprite.texture_atlas {
                atlas.index = row * 8 + col;
            }
        }
    }
}
