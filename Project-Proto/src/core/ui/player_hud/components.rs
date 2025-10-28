use bevy::prelude::*;
use bevy_egui::egui;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BodyPart {
    Head,
    Torso,
    Arms,
    Hands,
    Thighs,
    Calves,
}

#[derive(Resource)]
pub struct PlayerBodyState {
    pub head_hp: i32,
    pub torso_hp: i32,
    pub arms_hp: i32,
    pub hands_hp: i32,
    pub thighs_hp: i32,
    pub calves_hp: i32,
}

impl Default for PlayerBodyState {
    fn default() -> Self {
        Self {
            head_hp: 100,
            torso_hp: 100,
            arms_hp: 100,
            hands_hp: 100,
            thighs_hp: 100,
            calves_hp: 100,
        }
    }
}

impl PlayerBodyState {
    pub fn get_hp(&self, part: BodyPart) -> i32 {
        match part {
            BodyPart::Head => self.head_hp,
            BodyPart::Torso => self.torso_hp,
            BodyPart::Arms => self.arms_hp,
            BodyPart::Hands => self.hands_hp,
            BodyPart::Thighs => self.thighs_hp,
            BodyPart::Calves => self.calves_hp,
        }
    }

    pub fn hp_to_color(&self, part: BodyPart) -> egui::Color32 {
        let hp = self.get_hp(part);
        if hp <= 0 {
            // 사망 - 진한 회색
            egui::Color32::from_rgba_unmultiplied(60, 60, 60, 180)
        } else if hp < 30 {
            // 위험 - 빨간색
            egui::Color32::from_rgba_unmultiplied(200, 50, 50, 200)
        } else if hp < 70 {
            // 부상 - 노란색
            egui::Color32::from_rgba_unmultiplied(200, 180, 50, 200)
        } else {
            // 건강 - 흰색/밝은 회색
            egui::Color32::from_rgba_unmultiplied(220, 220, 220, 20)
        }
    }
}
