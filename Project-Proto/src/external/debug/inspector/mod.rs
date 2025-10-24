use bevy::prelude::*;
use bevy_egui::EguiPostUpdateSet;

use components::*;
use systems::*;

pub mod components;
mod systems;

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiState::new())
            .init_resource::<InspectorEnabled>()
            .add_systems(Update, toggle_inspector)
            .add_systems(
                PostUpdate,
                show_ui_system
                    .before(EguiPostUpdateSet::ProcessOutput)
                    .before(bevy_egui::end_pass_system)
                    .before(bevy::transform::TransformSystem::TransformPropagate)
                    .run_if(should_show_inspector),
            )
            .add_systems(Last, force_custom_cursor_silent);
    }
}
