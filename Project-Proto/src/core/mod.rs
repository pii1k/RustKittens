mod environment;

use bevy::prelude::*;

use environment::components::EnvironmentPlugin;

pub fn plugin(app: &mut App) {
    app.add_plugins(EnvironmentPlugin);
}
