use Project_Proto::GamePlugin;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::{DefaultInspectorConfigPlugin, quick::WorldInspectorPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Project: Proto".into(),
                resolution: (1920.0, 1080.0).into(),
                resizable: false,
                mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(EguiPlugin)
        .add_plugins(DefaultInspectorConfigPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(GamePlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
