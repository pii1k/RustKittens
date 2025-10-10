use bevy::prelude::*;

mod ui;

pub fn plugin(app: &mut App) {
    app.add_plugins(ui::editor::components::EditorPlugin);
}
