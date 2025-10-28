use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CustomCursorIcon {
    pub icon: Handle<Image>,
}
