use bevy::prelude::*;

use crate::{
    common::animation::components::{AnimationController, Direction8},
    core::player::{
        components::{Player, PlayerMovementState},
        systems::animation::setup_player_animation,
    },
};

const PLAYER_SCALE: f32 = 2.0;

pub fn spawn_player(
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
        AnimationController::new(PlayerMovementState::Idle, Direction8::South),
        Player::default(),
        anim_set,
    ));
}
