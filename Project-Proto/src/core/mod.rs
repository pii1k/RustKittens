mod environment;
mod player;

use bevy::prelude::*;

use environment::components::EnvironmentPlugin;
use player::components::PlayerPlugin;

pub fn plugin(app: &mut App) {
    app.add_plugins((EnvironmentPlugin, PlayerPlugin));
}
