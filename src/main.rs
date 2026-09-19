use bevy::prelude::*;

pub mod enemies;
pub mod physics;
pub mod player;
pub mod splitter_core;
pub mod ui;

pub mod camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Splitter".into(),
                // Fixed: Resolution now takes u32 integers instead of floats
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(player::PlayerPlugin)
        .add_systems(Startup, camera::setup_camera)
        .add_systems(Update, camera::fit_camera_viewport)
        .add_plugins(splitter_core::SplitterCorePlugin)
        .add_plugins(ui::UIPlugin)
        .add_plugins(enemies::EnemiesPlugin)
        .add_plugins(physics::SplitterPhysicsPlugin)
        .run();
}
