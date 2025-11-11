use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub velocity: Vec2,
    pub is_aiming: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerLifeStatus {
    Healthy,
    Injured,
    Critical,
    Destroyed,
}
