mod camera;
mod environment;
mod player;

use bevy::prelude::*;

use camera::components::CameraPlugin;
use environment::components::EnvironmentPlugin;
use player::components::PlayerPlugin;

pub fn plugin(app: &mut App) {
    app.add_plugins((EnvironmentPlugin, CameraPlugin, PlayerPlugin));
}
