use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Resource, Default)]
pub struct CursorCoords(pub Vec2);
