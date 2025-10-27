use bevy::prelude::*;

pub mod components;
mod systems;

use systems::setup_camera;
use systems::update_sticked_camera;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_camera)
            .add_systems(Update, update_sticked_camera);
    }
}
