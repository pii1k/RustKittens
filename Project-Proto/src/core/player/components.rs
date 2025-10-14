use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub velocity: Vec2,
    pub health: f32,
}

#[derive(Component, Clone, PartialEq, Eq, Hash)]
pub enum PlayerState {
    Idle,
    Walk,
    Attack,
    Hurt,
}
