mod common;
mod core;
mod external;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // WARN: 순서 바꾸지 말 것!
        app.add_plugins(common::plugin)
            .add_plugins(core::plugin)
            .add_plugins(external::plugin);
    }
}
