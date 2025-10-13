use bevy::{
    prelude::*,
    winit::cursor::{CursorIcon, CustomCursor},
};

use crate::common::cursor::components::CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cursor);
    }
}

pub fn setup_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<Entity, With<Window>>,
) {
    commands
        .entity(*window)
        .insert(CursorIcon::Custom(CustomCursor::Image {
            handle: asset_server.load("cursor/crosshair001.png"),
            hotspot: (32, 32),
        }));
}
