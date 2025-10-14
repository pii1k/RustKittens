use bevy::prelude::SystemSet;

pub mod components;
mod systems;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CameraSystemSet {
    CameraSetup,
}
