mod environment;
mod player;
mod ui;

use bevy::prelude::*;

use environment::components::EnvironmentPlugin;
use player::components::PlayerPlugin;
use ui::{components::GameState, UIPlugin};

pub fn plugin(app: &mut App) {
    app.init_state::<GameState>();
    app.add_plugins((EnvironmentPlugin, PlayerPlugin, UIPlugin));
}
