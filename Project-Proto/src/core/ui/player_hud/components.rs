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
    pub head_hp: f32,
    pub torso_hp: f32,
    pub arms_hp: f32,
    pub hands_hp: f32,
    pub thighs_hp: f32,
    pub calves_hp: f32,
}

impl Default for PlayerBodyState {
    fn default() -> Self {
        Self {
            head_hp: 1.0,
            torso_hp: 1.0,
            arms_hp: 1.0,
            hands_hp: 1.0,
            thighs_hp: 1.0,
            calves_hp: 1.0,
        }
    }
}

impl PlayerBodyState {
    pub fn get_hp(&self, part: BodyPart) -> f32 {
        match part {
            BodyPart::Head => self.head_hp,
            BodyPart::Torso => self.torso_hp,
            BodyPart::Arms => self.arms_hp,
            BodyPart::Hands => self.hands_hp,
            BodyPart::Thighs => self.thighs_hp,
            BodyPart::Calves => self.calves_hp,
        }
    }

    pub fn set_hp(&mut self, part: BodyPart, hp: f32) {
        let hp = hp.clamp(0.0, 1.0);
        match part {
            BodyPart::Head => self.head_hp = hp,
            BodyPart::Torso => self.torso_hp = hp,
            BodyPart::Arms => self.arms_hp = hp,
            BodyPart::Hands => self.hands_hp = hp,
            BodyPart::Thighs => self.thighs_hp = hp,
            BodyPart::Calves => self.calves_hp = hp,
        }
    }

    pub fn hp_to_color(&self, part: BodyPart) -> egui::Color32 {
        let hp = self.get_hp(part);
        if hp > 0.7 {
            egui::Color32::from_rgb(50, 200, 50) // 초록색 - 건강
        } else if hp > 0.3 {
            egui::Color32::from_rgb(230, 180, 50) // 노란색 - 부상
        } else if hp > 0.0 {
            egui::Color32::from_rgb(230, 50, 50) // 빨간색 - 위험
        } else {
            egui::Color32::from_rgb(80, 80, 80) // 회색 - 사망
        }
    }
}
