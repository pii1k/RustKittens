use bevy::prelude::*;

use crate::{
    common::{
        animation::components::{AnimationController, Direction8},
        camera::components::{StickedCameraComponent, StickedTargetComponent},
    },
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
    mut q: Query<&mut StickedCameraComponent, With<Camera2d>>,
) {
    let anim_set = setup_player_animation(&asset_server, &mut atlas_assets);
    let init_clip = anim_set.get_clip("idle").unwrap();

    let id = commands
        .spawn((
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
            Player {
                velocity: Vec2::ZERO,
                health: 100.0,
            },
            StickedTargetComponent,
            anim_set,
        ))
        .id();
    let mut cam = q.single_mut();
    cam.is_sticked = true;
    cam.target_index = id.index();
}
