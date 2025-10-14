use bevy::prelude::*;

use crate::{
    common::animation::systems::{animate_sprites, handle_animation_state_change},
    core::player::{
        components::PlayerState,
        systems::{
            animation::update_player_state,
            combat::{aim_at_cursor, shoot},
            life_cycle::spawn_player,
            movement::move_player,
        },
    },
};

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
                    handle_animation_state_change::<PlayerState>,
                    animate_sprites::<PlayerState>,
                ),
            )
            .add_systems(Update, (aim_at_cursor, shoot, move_player));
    }
}
