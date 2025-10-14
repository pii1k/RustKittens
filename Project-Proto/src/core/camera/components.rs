use bevy::prelude::*;

pub struct CameraPlugin;

#[derive(Component)]
pub struct StickedCameraComponent {
    pub target_index: u32,
    pub is_sticked: bool,
}

#[derive(Component)]
pub struct StickedTargetComponent;
