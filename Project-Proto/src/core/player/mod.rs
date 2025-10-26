use bevy::prelude::*;

use crate::common::animation::systems::*;
use components::*;
use systems::{animation::*, combat::*, fov::*, life_cycle::*, movement::*};

pub mod components;
mod systems;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (
                    update_player_state,
                    handle_animation_state_change::<PlayerMovementState>,
                    animate_sprites::<PlayerMovementState>,
                ),
            )
            .add_systems(Update, (aim_at_cursor, shoot, move_player, draw_fov));
    }
}
