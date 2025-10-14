pub mod animation;
pub mod cursor;

use bevy::prelude::*;

use crate::common::cursor::components::CursorPlugin;

pub fn plugin(app: &mut App) {
    app.add_plugins(CursorPlugin);
}
