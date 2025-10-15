use bevy::{
    prelude::*,
    winit::cursor::{CursorIcon, CustomCursor},
};

use crate::common::cursor::components::CustomCursorIcon;

pub fn setup_cursor(
    mut commands: Commands,
    mut custom_cursor_icon: ResMut<CustomCursorIcon>,
    asset_server: Res<AssetServer>,
    window: Single<Entity, With<Window>>,
) {
    let custom_cursor_image_handle: Handle<Image> = asset_server.load("cursor/crosshair001.png");

    commands
        .entity(*window)
        .insert(CursorIcon::Custom(CustomCursor::Image {
            handle: custom_cursor_image_handle.clone(),
            hotspot: (64, 64),
        }));

    custom_cursor_icon.icon = custom_cursor_image_handle.clone();
}
