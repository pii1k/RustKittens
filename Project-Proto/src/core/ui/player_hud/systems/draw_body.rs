use bevy_egui::egui::{self, *};

use crate::core::ui::player_hud::components::{BodyPart, PlayerBodyState};

pub fn draw_body_figure(painter: &Painter, state: &PlayerBodyState, center_x: f32, top_y: f32) {
    let outline_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, 100);
    let outline_stroke = egui::Stroke::new(1.0, outline_color);

    // 머리
    let head_center = egui::pos2(center_x, top_y + 12.0);
    let head_radius = 10.0;

    painter.circle_filled(head_center, head_radius, state.hp_to_color(BodyPart::Head));
    painter.circle_stroke(head_center, head_radius, outline_stroke);

    // 목
    painter.line_segment(
        [
            egui::pos2(center_x, top_y + 22.0),
            egui::pos2(center_x, top_y + 28.0),
        ],
        egui::Stroke::new(
            3.0,
            egui::Color32::from_rgba_unmultiplied(200, 200, 200, 150),
        ),
    );

    // 몸통
    let torso_rect =
        egui::Rect::from_center_size(egui::pos2(center_x, top_y + 55.0), egui::vec2(30.0, 40.0));
    painter.rect_filled(torso_rect, 2.0, state.hp_to_color(BodyPart::Torso));
    painter.rect_stroke(torso_rect, 2.0, outline_stroke, StrokeKind::Outside);

    // 팔
    let arm_color = state.hp_to_color(BodyPart::Arms);

    let left_arm = egui::Rect::from_min_size(
        egui::pos2(center_x - 38.0, top_y + 35.0),
        egui::vec2(8.0, 35.0),
    );
    painter.rect_filled(left_arm, 1.5, arm_color);
    painter.rect_stroke(left_arm, 1.5, outline_stroke, StrokeKind::Outside);

    let right_arm = egui::Rect::from_min_size(
        egui::pos2(center_x + 30.0, top_y + 35.0),
        egui::vec2(8.0, 35.0),
    );
    painter.rect_filled(right_arm, 1.5, arm_color);
    painter.rect_stroke(right_arm, 1.5, outline_stroke, StrokeKind::Outside);

    // 손
    let hand_color = state.hp_to_color(BodyPart::Hands);

    let left_hand = egui::Rect::from_min_size(
        egui::pos2(center_x - 38.0, top_y + 70.0),
        egui::vec2(8.0, 12.0),
    );
    painter.rect_filled(left_hand, 1.5, hand_color);
    painter.rect_stroke(left_hand, 1.5, outline_stroke, StrokeKind::Outside);

    // 오른손
    let right_hand = egui::Rect::from_min_size(
        egui::pos2(center_x + 30.0, top_y + 70.0),
        egui::vec2(8.0, 12.0),
    );
    painter.rect_filled(right_hand, 1.5, hand_color);
    painter.rect_stroke(right_hand, 1.5, outline_stroke, StrokeKind::Outside);

    // ===== 허벅지 (Thighs) =====
    let thigh_color = state.hp_to_color(BodyPart::Thighs);

    // 왼쪽 허벅지
    let left_thigh = egui::Rect::from_min_size(
        egui::pos2(center_x - 14.0, top_y + 75.0),
        egui::vec2(10.0, 40.0),
    );
    painter.rect_filled(left_thigh, 1.5, thigh_color);
    painter.rect_stroke(left_thigh, 1.5, outline_stroke, StrokeKind::Outside);

    // 오른쪽 허벅지
    let right_thigh = egui::Rect::from_min_size(
        egui::pos2(center_x + 4.0, top_y + 75.0),
        egui::vec2(10.0, 40.0),
    );
    painter.rect_filled(right_thigh, 1.5, thigh_color);
    painter.rect_stroke(right_thigh, 1.5, outline_stroke, StrokeKind::Outside);

    // ===== 종아리 (Calves) =====
    let calf_color = state.hp_to_color(BodyPart::Calves);

    // 왼쪽 종아리
    let left_calf = egui::Rect::from_min_size(
        egui::pos2(center_x - 14.0, top_y + 115.0),
        egui::vec2(10.0, 40.0),
    );
    painter.rect_filled(left_calf, 1.5, calf_color);
    painter.rect_stroke(left_calf, 1.5, outline_stroke, StrokeKind::Outside);

    // 오른쪽 종아리
    let right_calf = egui::Rect::from_min_size(
        egui::pos2(center_x + 4.0, top_y + 115.0),
        egui::vec2(10.0, 40.0),
    );
    painter.rect_filled(right_calf, 1.5, calf_color);
    painter.rect_stroke(right_calf, 1.5, outline_stroke, StrokeKind::Outside);
}
