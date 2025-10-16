use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use debug::inspector::EditorPlugin;

mod debug;

pub fn plugin(app: &mut App) {
    app.add_plugins((EguiPlugin, EditorPlugin));
}
