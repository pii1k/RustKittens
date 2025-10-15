use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use ui::inspector::EditorPlugin;

mod ui;

pub fn plugin(app: &mut App) {
    app.add_plugins((EguiPlugin, EditorPlugin));
}
