pub mod animation;
pub mod camera;
pub mod cursor;

use bevy::prelude::*;

use camera::CameraPlugin;
use cursor::CursorPlugin;

pub fn plugin(app: &mut App) {
    app.add_plugins(CursorPlugin).add_plugins(CameraPlugin);
}
