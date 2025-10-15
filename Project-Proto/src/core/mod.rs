mod environment;
mod player;

use bevy::prelude::*;

use environment::EnvironmentPlugin;
use player::PlayerPlugin;

pub fn plugin(app: &mut App) {
    app.add_plugins((EnvironmentPlugin, PlayerPlugin));
}
