use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct CursorAsset {
    pub image_handle: Handle<Image>,
    pub is_set: bool,
}

#[derive(Resource, Default)]
pub struct CursorCoords(pub Vec2);
