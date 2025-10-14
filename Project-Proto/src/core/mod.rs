mod environment;
mod player;

use bevy::prelude::*;

use crate::core::{environment::EnvironmentPlugin, player::PlayerPlugin};

pub fn plugin(app: &mut App) {
    app.add_plugins((EnvironmentPlugin, PlayerPlugin));
}
