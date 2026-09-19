use bevy::prelude::*;

pub mod player;
pub mod splitter_core;
pub mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Splitter".into(),
                // Fixed: Resolution now takes u32 integers instead of floats
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(player::PlayerPlugin)
        .add_systems(Startup, setup_camera)
        .add_plugins(splitter_core::SplitterCorePlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Fixed: Camera2dBundle was removed; Camera2d is now used directly
    commands.spawn(Camera2d);
}
