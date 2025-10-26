use bevy::prelude::*;

use crate::common::animation::components::AnimationState;

#[derive(Component)]
pub struct Player {
    pub fov_range: f32,
    pub fov_angle_in_rad: f32, // Radian 값, 예시) 150도 -> 150 * PI / 180 => Radian 값
    pub velocity: Vec2,
    pub health: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            fov_angle_in_rad: 150.0_f32.to_radians(),
            fov_range: 500.0,
            velocity: Vec2::ZERO,
            health: 100.0,
        }
    }
}

#[derive(Component, Clone, PartialEq, Eq, Hash)]
pub enum PlayerMovementState {
    Idle,
    Walk,
    Hurt,
}

impl AnimationState for PlayerMovementState {
    fn clip_name(&self) -> &str {
        match self {
            PlayerMovementState::Idle => "idle",
            PlayerMovementState::Walk => "walk",
            PlayerMovementState::Hurt => "hurt",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerLifeStatus {
    Healthy,
    Injured,
    Critical,
    Destroyed,
}
