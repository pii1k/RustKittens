use bevy::prelude::*;

use crate::{
    common::animation::components::AnimationController,
    core::player::{
        components::{Player, PlayerPlugin},
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
            .add_systems(Update, (aim_at_cursor, shoot, move_player));
    }
}

fn spawn_player(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(64, 64), 8, 8, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    commands.spawn((
        Name::new("Player"),
        Sprite {
            image: asset_server.load("player/walk.png"),
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle,
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(PLAYER_SCALE)),
        AnimationController::default(),
        Player,
    ));
}
