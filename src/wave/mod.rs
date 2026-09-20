pub mod definitions;
pub mod spawner;
pub mod wave_form;

use crate::{GameState, GameplaySet};
use bevy::prelude::*;
pub use definitions::{EnemySpawnerFn, WaveDefinition, WaveEnemyGroup, WaveState};
use spawner::{track_enemy_stats, update_wave_spawner};
use wave_form::auto_wave_advancement_system;

pub struct SplitterWavePlugin;

fn reset_wave_state(mut wave_state: ResMut<WaveState>) {
    *wave_state = WaveState::default();
}

impl Plugin for SplitterWavePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WaveState>()
            // 1. Re-initialize WaveState resource whenever transitioning into gameplay
            .add_systems(
                OnTransition {
                    exited: GameState::MainMenu,
                    entered: GameState::InGame,
                },
                reset_wave_state,
            )
            // 2. Tick wave systems only while active in GameState::InGame
            .add_systems(
                Update,
                (
                    update_wave_spawner,
                    track_enemy_stats,
                    auto_wave_advancement_system,
                )
                    .in_set(GameplaySet),
            );
    }
}
