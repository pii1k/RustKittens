mod environment;
mod player;
mod ui;

use bevy::prelude::*;

use environment::EnvironmentPlugin;
use player::PlayerPlugin;
use ui::player_hud::PlayerHUDPlugin;

pub fn plugin(app: &mut App) {
    app.add_plugins((EnvironmentPlugin, PlayerPlugin, PlayerHUDPlugin));
}
