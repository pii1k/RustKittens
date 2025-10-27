use avian2d::prelude::*;
use bevy::prelude::*;
use project_proto::GamePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Project: Proto".into(),
                        resolution: (1920.0, 1080.0).into(),
                        resizable: true,
                        mode: bevy::window::WindowMode::Windowed,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(GamePlugin)
        .run();
}
