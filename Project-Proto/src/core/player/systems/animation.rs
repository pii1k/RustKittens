use bevy::prelude::*;

use crate::common::animation::components::{AnimationClip, AnimationSet};

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

    let attack_fire_layout = atlas_assets.add(TextureAtlasLayout::from_grid(
        UVec2::new(64, 64),
        8,
        8,
        None,
        None,
    ));
    anim_set.add_clip(
        "attack_fire",
        AnimationClip {
            image_handle: asset_server.load("player/attack_fire.png"),
            texture_layout_handle: attack_fire_layout,
            frames_per_direction: 8,
            frame_duration: 0.05,
            looping: false,
        },
    );

    let attack_stock_layout = atlas_assets.add(TextureAtlasLayout::from_grid(
        UVec2::new(64, 64),
        8,
        8,
        None,
        None,
    ));
    anim_set.add_clip(
        "attack_stock",
        AnimationClip {
            image_handle: asset_server.load("player/attack_stock.png"),
            texture_layout_handle: attack_stock_layout,
            frames_per_direction: 8,
            frame_duration: 0.075,
            looping: false,
        },
    );

    let attack_run_layout = atlas_assets.add(TextureAtlasLayout::from_grid(
        UVec2::new(64, 64),
        8,
        8,
        None,
        None,
    ));
    anim_set.add_clip(
        "attack_run",
        AnimationClip {
            image_handle: asset_server.load("player/attack_run.png"),
            texture_layout_handle: attack_run_layout,
            frames_per_direction: 8,
            frame_duration: 0.075,
            looping: false,
        },
    );

    anim_set
}
