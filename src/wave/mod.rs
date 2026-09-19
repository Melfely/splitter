pub mod definitions;
pub mod spawner;
pub mod wave_form;

use crate::GameplaySet;
use bevy::prelude::*;
pub use definitions::{EnemySpawnerFn, WaveDefinition, WaveEnemyGroup, WaveState};
use spawner::{track_enemy_stats, update_wave_spawner};
use wave_form::auto_wave_advancement_system;

pub struct SplitterWavePlugin;

impl Plugin for SplitterWavePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WaveState>().add_systems(
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
