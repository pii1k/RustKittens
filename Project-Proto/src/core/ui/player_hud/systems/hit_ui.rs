use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{Align2, Color32, Sense, Window, vec2},
};

use crate::core::ui::player_hud::{
    components::PlayerBodyState, systems::draw_body::draw_body_figure,
};

pub fn player_hit_ui_system(mut contexts: EguiContexts, body_state: Res<PlayerBodyState>) {
    Window::new("Body Status")
        .anchor(Align2::RIGHT_BOTTOM, vec2(-20.0, -20.0))
        .resizable(false)
        .collapsible(false)
        .default_width(120.0)
        .show(contexts.ctx_mut(), |ui| {
            let (response, painter) = ui.allocate_painter(vec2(160.0, 240.0), Sense::hover());

            let rect = response.rect;
            let center_x = rect.center().x;
            let top_y = rect.top() + 10.0;

            painter.rect_filled(rect, 4.0, Color32::from_rgba_unmultiplied(20, 20, 20, 200));

            draw_body_figure(&painter, &body_state, center_x, top_y);
        });
}
