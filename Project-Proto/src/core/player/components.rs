use bevy::prelude::*;

use crate::common::animation::components::AnimationState;

#[derive(Component)]
pub struct Player {
    pub velocity: Vec2,
    pub health: f32,
}

#[derive(Component, Clone, PartialEq, Eq, Hash)]
pub enum PlayerState {
    Idle,
    Walk,
    Hurt,
}

impl AnimationState for PlayerState {
    fn clip_name(&self) -> &str {
        match self {
            PlayerState::Idle => "idle",
            PlayerState::Walk => "walk",
            PlayerState::Hurt => "hurt",
        }
    }
}
