use bevy::{
    prelude::*,
    window::PrimaryWindow,
    winit::cursor::{CursorIcon, CustomCursor},
};

use crate::core::player::components::CursorCoords;

use super::components::{Player, PlayerPlugin};

const PLAYER_SIZE: Vec2 = Vec2::new(32.0, 64.0);
const PLAYER_SPEED: f32 = 350.0;
const PLAYER_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorCoords::default())
            .add_systems(Startup, (spawn_player, setup_cursor))
            .add_systems(Update, (move_player, aim_at_cursor, shoot))
            .add_systems(
                PostUpdate,
                update_cursor_world_position.after(TransformSystem::TransformPropagate),
            )
            .add_systems(Last, force_custom_cursor_silent);
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Name::new("Player"),
        Sprite::from_color(PLAYER_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: PLAYER_SIZE.extend(1.0),
            ..default()
        },
        Player,
    ));
}

fn setup_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<Entity, With<PrimaryWindow>>,
) {
    commands
        .entity(*window)
        .insert(CursorIcon::Custom(CustomCursor::Image {
            handle: asset_server.load("cursor/crosshair019.png"),
            hotspot: (0, 0),
        }));
}

fn force_custom_cursor_silent(
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut CursorIcon, With<Window>>,
) {
    for mut cursor_icon in windows.iter_mut() {
        // bypass_change_detection으로 Change 이벤트 없이 설정
        if !matches!(*cursor_icon.as_ref(), CursorIcon::Custom(_)) {
            *cursor_icon.bypass_change_detection() = CursorIcon::Custom(CustomCursor::Image {
                handle: asset_server.load("cursor/crosshair019.png"),
                hotspot: (16, 16),
            });
        }
    }
}
fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_transform: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut direction = Vec2::new(0.0, 0.0);

    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 0.5;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 0.5;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }

    let movement = direction * PLAYER_SPEED * time.delta_secs();
    player_transform.translation += movement.extend(0.0);
}

fn update_cursor_world_position(
    mut cursor_coords: ResMut<CursorCoords>,
    q_window: Single<&Window, With<PrimaryWindow>>,
    q_camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let (camera, camera_transform) = *q_camera;

    if let Some(cursor_pos) = q_window.cursor_position() {
        cursor_coords.0 = camera
            .viewport_to_world_2d(camera_transform, cursor_pos)
            .unwrap_or(Vec2::ZERO);
    }
}

fn aim_at_cursor(
    q_player: Single<(&mut Transform, &GlobalTransform), With<Player>>,
    cursor_coords: Res<CursorCoords>,
) {
    let (mut transform, global_transform) = q_player.into_inner();
    let player_pos = global_transform.translation().truncate();
    let cursor_pos = cursor_coords.0;

    let directrion_to_cursor = cursor_pos - player_pos;
    let angle = directrion_to_cursor.y.atan2(directrion_to_cursor.x);

    transform.rotation = Quat::from_rotation_z(angle);
}

fn shoot(cursor_coords: Res<CursorCoords>, mouse_input: Res<ButtonInput<MouseButton>>) {
    if mouse_input.just_pressed(MouseButton::Left) {
        println!("Aiming at {}. Shot's fired!!", cursor_coords.0)
    }
}
