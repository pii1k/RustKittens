use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::core::{
    player::components::PlayerLifeStatus,
    ui::player_hud::{components::BodyPart, systems::draw_body::draw_body_figure},
};

use super::super::components::PlayerBodyState;

pub fn player_hit_ui_system(mut contexts: EguiContexts, body_state: Res<PlayerBodyState>) {
    let ctx = contexts.ctx_mut();

    egui::Area::new(egui::Id::new("player_body_hud"))
        .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-30.0, -15.0))
        .show(ctx, |ui| {
            ui.set_clip_rect(ui.max_rect());

            let (response, painter) =
                ui.allocate_painter(egui::vec2(100.0, 200.0), egui::Sense::hover());

            let rect = response.rect;
            let center_x = rect.center().x;
            let top_y = rect.top();

            draw_body_figure(&painter, &body_state, center_x, top_y);

            response.on_hover_ui(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new("BODY STATUS")
                            .strong()
                            .color(egui::Color32::WHITE),
                    );
                    ui.separator();

                    for (name, part) in [
                        ("Head", BodyPart::Head),
                        ("Torso", BodyPart::Torso),
                        ("Arms", BodyPart::Arms),
                        ("Hands", BodyPart::Hands),
                        ("Thighs", BodyPart::Thighs),
                        ("Calves", BodyPart::Calves),
                    ] {
                        let hp = body_state.get_hp(part);
                        let status = if hp > 70 {
                            PlayerLifeStatus::Healthy
                        } else if hp > 30 {
                            PlayerLifeStatus::Injured
                        } else if hp > 0 {
                            PlayerLifeStatus::Critical
                        } else {
                            PlayerLifeStatus::Destroyed
                        };

                        ui.horizontal(|ui| {
                            ui.label(format!("{name:8}"));
                            ui.label(format!("{hp}%"));
                            ui.label(
                                egui::RichText::new(format!("[{status:?}]"))
                                    .color(hp_to_status_color(hp)),
                            );
                        });
                    }
                });
            });
        });
}

fn hp_to_status_color(hp: i32) -> egui::Color32 {
    if hp > 70 {
        egui::Color32::from_rgb(100, 220, 100)
    } else if hp > 30 {
        egui::Color32::from_rgb(220, 200, 80)
    } else if hp > 0 {
        egui::Color32::from_rgb(220, 80, 80)
    } else {
        egui::Color32::from_rgb(120, 120, 120)
    }
}
