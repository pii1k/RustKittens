use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub speed: f32,
    pub size: Vec2,
    pub lifetime: Timer,
}

impl Default for Projectile {
    fn default() -> Self {
        Self {
            speed: 750.0,
            size: Vec2::new(20.0, 8.0),
            lifetime: Timer::from_seconds(5.0, TimerMode::Once),
        }
    }
}
