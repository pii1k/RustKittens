pub mod components;
mod systems;

use bevy::prelude::*;

use crate::core::ui::player_hud::{
    components::PlayerBodyState, systems::hit_ui::player_hit_ui_system,
};

pub struct PlayerHUDPlugin;
impl Plugin for PlayerHUDPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerBodyState>()
            .add_systems(Update, player_hit_ui_system);
    }
}
