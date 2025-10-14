use bevy::prelude::*;

use crate::{
    common::animation::{
        components::{
            AnimationClip, AnimationController, AnimationSet, AnimationState, Direction8,
        },
        systems::base::{animate_sprites, handle_animation_state_change},
    },
    core::player::{
        components::{Player, PlayerPlugin, PlayerState},
        systems::{
            combat::{aim_at_cursor, shoot},
            movement::move_player,
        },
    },
};

const PLAYER_SCALE: f32 = 2.0;

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

impl AnimationState for PlayerState {
    fn clip_name(&self) -> &str {
        match self {
            PlayerState::Idle => "idle",
            PlayerState::Walk => "walk",
            PlayerState::Attack => "attack",
            PlayerState::Hurt => "hurt",
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    mut atlas_assets: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    let anim_set = setup_player_animation(&asset_server, &mut atlas_assets);
    let init_clip = anim_set.get_clip("idle").unwrap();

    commands.spawn((
        Name::new("Player"),
        Sprite {
            image: asset_server.load("player/walk.png"),
            texture_atlas: Some(TextureAtlas {
                layout: init_clip.texture_layout_handle.clone(),
                index: 4,
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(PLAYER_SCALE)),
        AnimationController::new(PlayerState::Idle, Direction8::South),
        Player {
            velocity: Vec2::ZERO,
            health: 100.0,
        },
        anim_set,
    ));
}

fn update_player_state(
    q_player: Single<(&Player, &mut AnimationController<PlayerState>), With<Player>>,
) {
    let (player, mut anim_controller) = q_player.into_inner();

    if player.health < 50.0 && anim_controller.state != PlayerState::Hurt {
        anim_controller.state = PlayerState::Hurt;
    }

    if player.velocity != Vec2::ZERO {
        if anim_controller.state != PlayerState::Walk {
            anim_controller.state = PlayerState::Walk;
        }
    } else if anim_controller.state != PlayerState::Idle {
        anim_controller.state = PlayerState::Idle;
    }
}

fn setup_player_animation(
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
