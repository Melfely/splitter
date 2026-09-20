use bevy::prelude::*;

pub mod camera;
pub mod effects;
pub mod enemies;
pub mod menu;
pub mod physics;
pub mod player;
pub mod splitter_core;
pub mod ui;
pub mod wave;

use menu::GameState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct GameplaySet;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, GameplaySet.run_if(in_state(GameState::InGame)))
            .add_plugins((
                splitter_core::SplitterCorePlugin, //[cite: 1]
                player::PlayerPlugin,              //[cite: 1]
                enemies::EnemiesPlugin,            //[cite: 2]
                physics::SplitterPhysicsPlugin,    //[cite: 4]
                wave::SplitterWavePlugin,
                ui::UIPlugin,
                effects::EffectsPlugin,
            ));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Splitter".into(),
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((menu::SplitterMenuPlugin, GameplayPlugin))
        .add_systems(Startup, camera::setup_camera)
        .add_systems(Update, camera::fit_camera_viewport)
        .run();
}
