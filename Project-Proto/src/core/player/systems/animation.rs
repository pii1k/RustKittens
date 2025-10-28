use bevy::prelude::*;

use crate::{
    common::animation::components::{AnimationClip, AnimationController, AnimationSet},
    core::player::components::{Player, PlayerMovementState},
};

pub fn setup_player_animation(
    asset_server: &AssetServer,
    atlas_assets: &mut Assets<TextureAtlasLayout>,
) -> AnimationSet {
    let mut anim_set = AnimationSet::new();

    let idle_layout = atlas_assets.add(TextureAtlasLayout::from_grid(
        UVec2::new(64, 64),
        8,
        8,
        None,
        None,
    ));
    anim_set.add_clip(
        "idle",
        AnimationClip {
            image_handle: asset_server.load("player/idle.png"),
            texture_layout_handle: idle_layout,
            frames_per_direction: 8,
            frame_duration: 0.1,
            looping: true,
        },
    );

    let walk_layout = atlas_assets.add(TextureAtlasLayout::from_grid(
        UVec2::new(64, 64),
        8,
        8,
        None,
        None,
    ));
    anim_set.add_clip(
        "walk",
        AnimationClip {
            image_handle: asset_server.load("player/walk.png"),
            texture_layout_handle: walk_layout,
            frames_per_direction: 8,
            frame_duration: 0.1,
            looping: true,
        },
    );

    let attack_layout = atlas_assets.add(TextureAtlasLayout::from_grid(
        UVec2::new(64, 64),
        8,
        8,
        None,
        None,
    ));
    anim_set.add_clip(
        "attack",
        AnimationClip {
            image_handle: asset_server.load("player/attack.png"),
            texture_layout_handle: attack_layout,
            frames_per_direction: 8,
            frame_duration: 0.1,
            looping: false,
        },
    );

    anim_set
}

pub fn update_player_state(
    q_player: Single<(&Player, &mut AnimationController<PlayerMovementState>), With<Player>>,
) {
    let (player, mut anim_controller) = q_player.into_inner();

    if player.health < 50.0 && anim_controller.state != PlayerMovementState::Hurt {
        anim_controller.state = PlayerMovementState::Hurt;
    }

    if player.velocity != Vec2::ZERO {
        if anim_controller.state != PlayerMovementState::Walk {
            anim_controller.state = PlayerMovementState::Walk;
        }
    } else if anim_controller.state != PlayerMovementState::Idle {
        anim_controller.state = PlayerMovementState::Idle;
    }
}
