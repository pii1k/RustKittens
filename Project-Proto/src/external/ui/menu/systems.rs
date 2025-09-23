use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui::Window};

use crate::external::ui::menu::components::MenuUIPlugin;

impl Plugin for MenuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, main_menu_ui);
    }
}

fn main_menu_ui(mut contexts: EguiContexts) {
    Window::new("Proto").show(contexts.ctx_mut(), |ui| {
        ui.label("Wassup");
    });
}
