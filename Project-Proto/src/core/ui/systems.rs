use crate::core::ui::components::GameState;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

pub fn handle_menu_hotkeys(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }

    match state.get() {
        GameState::InGame => next_state.set(GameState::Paused),
        GameState::Paused => next_state.set(GameState::InGame),
        GameState::MainMenu => {}
    }
}

pub fn main_menu_ui(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);
            ui.heading(
                egui::RichText::new("Project Proto")
                    .size(48.0)
                    .color(egui::Color32::WHITE),
            );
            ui.add_space(50.0);

            if ui
                .add_sized([200.0, 50.0], egui::Button::new("Play"))
                .clicked()
            {
                next_state.set(GameState::InGame);
            }

            ui.add_space(20.0);
            if ui
                .add_sized([200.0, 50.0], egui::Button::new("Quit"))
                .clicked()
            {
                exit.send(AppExit::Success);
            }
        });
    });
}

pub fn pause_ui(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    let ctx = contexts.ctx_mut();
    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(egui::Color32::from_rgba_unmultiplied(0, 0, 0, 140)))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(60.0);
                egui::Frame::new()
                    .fill(egui::Color32::from_rgba_unmultiplied(24, 24, 24, 220))
                    .corner_radius(16)
                    .inner_margin(egui::Margin::symmetric(24, 24))
                    .show(ui, |ui| {
                        ui.set_width(320.0);

                        ui.heading(
                            egui::RichText::new("Paused")
                                .size(40.0)
                                .color(egui::Color32::WHITE),
                        );
                        ui.add_space(24.0);

                        if ui
                            .add_sized([240.0, 48.0], egui::Button::new("Continue"))
                            .clicked()
                        {
                            next_state.set(GameState::InGame);
                        }

                        ui.add_space(16.0);
                        if ui
                            .add_sized([240.0, 48.0], egui::Button::new("Main Menu"))
                            .clicked()
                        {
                            next_state.set(GameState::MainMenu);
                        }

                        ui.add_space(16.0);
                        if ui
                            .add_sized([240.0, 48.0], egui::Button::new("Quit"))
                            .clicked()
                        {
                            exit.send(AppExit::Success);
                        }
                    });
                ui.add_space(60.0);
            });
        });
}
