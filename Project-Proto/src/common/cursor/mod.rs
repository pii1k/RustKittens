use bevy::prelude::*;

use crate::common::cursor::systems::setup_cursor;

mod systems;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cursor);
    }
}
