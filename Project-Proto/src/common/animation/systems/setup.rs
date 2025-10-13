use bevy::prelude::*;

use crate::common::animation::{components::AnimationPlugin, systems::base::update_animations};

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_animations);
    }
}
