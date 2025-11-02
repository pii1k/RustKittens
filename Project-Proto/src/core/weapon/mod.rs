pub mod components;
mod systems;

use bevy::prelude::*;
use systems::update_projectiles;

pub struct WeaponPlugin;
impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_projectiles);
    }
}
