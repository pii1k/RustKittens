use super::components::*;
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d).insert(StickedCameraComponent {
        target_index: 0,
        is_sticked: false,
    });
}

#[allow(clippy::type_complexity)]
pub fn update_sticked_camera(
    time: Res<Time>,
    mut queries: ParamSet<(
        Query<(Entity, &Transform), With<StickedTargetComponent>>,
        Query<(&StickedCameraComponent, &mut Transform), With<Camera2d>>,
    )>,
) {
    let maybe_player_data = queries
        .p0()
        .get_single()
        .ok()
        .map(|(entity, transform)| (entity.index(), transform.translation));

    let Some((player_index, player_translation)) = maybe_player_data else {
        return;
    };

    for (cam_info, mut cam_transform) in queries.p1().iter_mut() {
        if cam_info.is_sticked && cam_info.target_index == player_index {
            let desired = Vec3::new(
                player_translation.x,
                player_translation.y,
                cam_transform.translation.z,
            );
            let damping = 6.0;
            cam_transform.translation = cam_transform
                .translation
                .lerp(desired, 1.0 - f32::exp(-damping * time.delta_secs()));
        }
    }
}
