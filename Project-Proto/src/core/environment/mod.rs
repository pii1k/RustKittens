use bevy::prelude::*;

use crate::core::environment::systems::setup_environment;

pub mod components;
mod systems;

pub struct EnvironmentPlugin;
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_environment);
    }
}
