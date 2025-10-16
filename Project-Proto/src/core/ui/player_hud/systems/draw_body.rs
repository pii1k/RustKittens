use bevy_egui::egui::*;

use crate::core::ui::player_hud::components::{BodyPart, PlayerBodyState};

pub fn draw_body_figure(painter: &Painter, state: &PlayerBodyState, center_x: f32, top_y: f32) {
    let stroke = Stroke::new(1.5, Color32::from_gray(100));

    // ===== 머리 (Head) =====
    let head_center = pos2(center_x, top_y + 15.0);
    let head_radius = 12.0;
    painter.circle(
        head_center,
        head_radius,
        state.hp_to_color(BodyPart::Head),
        stroke,
    );

    // ===== 목 (연결선) =====
    let neck_start = pos2(center_x, top_y + 27.0);
    let neck_end = pos2(center_x, top_y + 35.0);
    painter.line_segment(
        [neck_start, neck_end],
        Stroke::new(4.0, Color32::from_gray(150)),
    );

    // ===== 몸통 (Torso) =====
    let torso_rect = Rect::from_center_size(pos2(center_x, top_y + 65.0), vec2(40.0, 50.0));
    painter.rect(
        torso_rect,
        3.0,
        state.hp_to_color(BodyPart::Torso),
        stroke,
        StrokeKind::Outside,
    );

    // ===== 팔 (Arms) =====
    // 왼팔
    let left_arm_rect = Rect::from_min_size(pos2(center_x - 50.0, top_y + 45.0), vec2(10.0, 40.0));
    painter.rect(
        left_arm_rect,
        2.0,
        state.hp_to_color(BodyPart::Arms),
        stroke,
        StrokeKind::Outside,
    );

    // 오른팔
    let right_arm_rect = Rect::from_min_size(pos2(center_x + 40.0, top_y + 45.0), vec2(10.0, 40.0));
    painter.rect(
        right_arm_rect,
        2.0,
        state.hp_to_color(BodyPart::Arms),
        stroke,
        StrokeKind::Outside,
    );

    // ===== 손 (Hands) =====
    // 왼손
    let left_hand_rect = Rect::from_min_size(pos2(center_x - 50.0, top_y + 85.0), vec2(10.0, 15.0));
    painter.rect(
        left_hand_rect,
        2.0,
        state.hp_to_color(BodyPart::Hands),
        stroke,
        StrokeKind::Outside,
    );

    // 오른손
    let right_hand_rect =
        Rect::from_min_size(pos2(center_x + 40.0, top_y + 85.0), vec2(10.0, 15.0));
    painter.rect(
        right_hand_rect,
        2.0,
        state.hp_to_color(BodyPart::Hands),
        stroke,
        StrokeKind::Outside,
    );

    // ===== 허벅지 (Thighs) =====
    // 왼쪽 허벅지
    let left_thigh_rect =
        Rect::from_min_size(pos2(center_x - 18.0, top_y + 90.0), vec2(12.0, 45.0));
    painter.rect(
        left_thigh_rect,
        2.0,
        state.hp_to_color(BodyPart::Thighs),
        stroke,
        StrokeKind::Outside,
    );

    // 오른쪽 허벅지
    let right_thigh_rect =
        Rect::from_min_size(pos2(center_x + 6.0, top_y + 90.0), vec2(12.0, 45.0));
    painter.rect(
        right_thigh_rect,
        2.0,
        state.hp_to_color(BodyPart::Thighs),
        stroke,
        StrokeKind::Outside,
    );

    // ===== 종아리 (Calves) =====
    // 왼쪽 종아리
    let left_calf_rect =
        Rect::from_min_size(pos2(center_x - 18.0, top_y + 135.0), vec2(12.0, 45.0));
    painter.rect(
        left_calf_rect,
        2.0,
        state.hp_to_color(BodyPart::Calves),
        stroke,
        StrokeKind::Outside,
    );

    // 오른쪽 종아리
    let right_calf_rect =
        Rect::from_min_size(pos2(center_x + 6.0, top_y + 135.0), vec2(12.0, 45.0));
    painter.rect(
        right_calf_rect,
        2.0,
        state.hp_to_color(BodyPart::Calves),
        stroke,
        StrokeKind::Outside,
    );
}
