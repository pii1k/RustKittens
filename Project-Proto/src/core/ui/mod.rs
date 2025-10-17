pub mod components;
mod systems;

use bevy::prelude::*;
use components::GameState;
use crate::core::ui::systems::{handle_menu_hotkeys, main_menu_ui, pause_ui};

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_menu_hotkeys,
                main_menu_ui.run_if(in_state(GameState::MainMenu)),
                pause_ui.run_if(in_state(GameState::Paused)),
            ),
        );
    }
}