mod common;
mod core;
mod external;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((core::plugin, common::plugin));
        app.add_plugins((external::plugin, core::plugin));
    }
}
