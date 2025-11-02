mod environment;
mod player;
mod ui;
mod weapon;

use bevy::prelude::*;

use environment::EnvironmentPlugin;
use player::PlayerPlugin;
use ui::player_hud::PlayerHUDPlugin;
use weapon::WeaponPlugin;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        EnvironmentPlugin,
        PlayerPlugin,
        PlayerHUDPlugin,
        WeaponPlugin,
    ));
}
