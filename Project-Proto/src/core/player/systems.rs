use bevy::prelude::*;

use super::components::{Player, PlayerPlugin};

const PLAYER_SIZE: Vec2 = Vec2::new(40.0, 60.0);
const PLAYER_SPEED: f32 = 350.0;
const PLAYER_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player, shoot));
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(PLAYER_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: PLAYER_SIZE.extend(1.0),
            ..default()
        },
        Player,
    ));
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
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }

    let movement = direction * PLAYER_SPEED * time.delta_secs();
    player_transform.translation += movement.extend(0.0);
}

fn shoot(mouse_input: Res<ButtonInput<MouseButton>>) {
    if mouse_input.just_pressed(MouseButton::Left) {
        println!("Shoot!!")
    }
}
