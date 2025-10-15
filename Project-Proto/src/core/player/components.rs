use bevy::prelude::*;

use crate::common::animation::components::AnimationState;

#[derive(Component)]
pub struct Player {
    pub velocity: Vec2,
    pub health: f32,
}

#[derive(Resource, Default)]
pub struct CursorAsset {
    pub image_handle: Handle<Image>,
    pub is_set: bool,
}

#[derive(Resource, Default)]
pub struct CursorCoords(pub Vec2);
