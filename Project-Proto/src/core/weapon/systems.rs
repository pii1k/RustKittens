use bevy::prelude::*;

use super::components::Projectile;

pub fn update_projectiles(
    time: Res<Time>,
    mut commands: Commands,
    mut q_projectiles: Query<(Entity, &mut Projectile)>,
) {
    for (entity, mut proj) in &mut q_projectiles {
        proj.lifetime.tick(time.delta());

        if proj.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
