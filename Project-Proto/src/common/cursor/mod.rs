use bevy::prelude::*;

use components::*;
use systems::*;

pub mod components;
mod systems;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CustomCursorIcon::default())
            .add_systems(PreStartup, setup_cursor);
    }
}
