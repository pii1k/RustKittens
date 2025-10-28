use bevy::prelude::*;

use crate::common::animation::components::AnimationState;

#[derive(Component)]
pub struct Player {
    pub velocity: Vec2,
    pub health: f32,
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
