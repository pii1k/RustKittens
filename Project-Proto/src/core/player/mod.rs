use crate::common::animation::systems::*;
use bevy::prelude::*;
use components::*;
use systems::{animation::*, combat::*, life_cycle::*, movement::*};

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
            .add_systems(Update, (move_player, aim_at_cursor, shoot).chain());
    }
}
