use super::definitions::{WaveDefinition, WaveEnemyGroup, WaveState};
use crate::enemies::types::{
    spawn_fast_red_charger, spawn_fast_red_charger_special, spawn_heavy_yellow_tank,
    spawn_heavy_yellow_tank_special,
};
use crate::menu::GameState;
use bevy::prelude::*;
use rand::RngExt;

/// Generates a `WaveDefinition` for waves 1 through 20 based on wave scaling rules.
/// Rolls a 1/10 (10%) chance for each individual enemy to spawn as its special variant.
pub fn generate_wave_definition(wave_number: u32) -> Option<WaveDefinition> {
    if wave_number < 1 || wave_number > 20 {
        return None;
    }

    let mut rng = rand::rng();

    // Fast red chargers start at 10 and increase by 10 per wave number
    let total_red_chargers = (wave_number * 10) as usize;
    let mut regular_red_count = 0;
    let mut special_red_count = 0;

    for _ in 0..total_red_chargers {
        if rng.random_range(0..10) == 0 {
            special_red_count += 1;
        } else {
            regular_red_count += 1;
        }
    }

    let mut groups = Vec::new();

    if regular_red_count > 0 {
        groups.push(WaveEnemyGroup {
            spawner: spawn_fast_red_charger,
            count: regular_red_count,
        });
    }

    if special_red_count > 0 {
        groups.push(WaveEnemyGroup {
            spawner: spawn_fast_red_charger_special,
            count: special_red_count,
        });
    }

    // Waves 11 through 20 include heavy yellow tanks
    if (11..=20).contains(&wave_number) {
        let total_tanks = 1;
        let mut regular_tank_count = 0;
        let mut special_tank_count = 0;

        for _ in 0..total_tanks {
            if rng.random_range(0..10) == 0 {
                special_tank_count += 1;
            } else {
                regular_tank_count += 1;
            }
        }

        if regular_tank_count > 0 {
            groups.push(WaveEnemyGroup {
                spawner: spawn_heavy_yellow_tank,
                count: regular_tank_count,
            });
        }

        if special_tank_count > 0 {
            groups.push(WaveEnemyGroup {
                spawner: spawn_heavy_yellow_tank_special,
                count: special_tank_count,
            });
        }
    }

    // Batch size starts at 5 for wave 1 and ramps up by +1 per wave number
    let batch_size = (5 + (wave_number - 1)) as usize;

    Some(WaveDefinition {
        wave_number,
        groups,
        spawn_interval: 0.25,
        batch_size,
    })
}

/// System that manages wave lifecycle:
/// - Starts the next wave when returning to `GameState::InGame` or on initial start.
/// - Detects wave completion and transitions to `GameState::CardSelect`.
pub fn auto_wave_advancement_system(
    mut wave_state: ResMut<WaveState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if wave_state.wave_in_progress {
        // Wave active: trigger card selection menu when all spawns are complete and enemies killed
        if wave_state.pending_spawns.is_empty() && wave_state.current_enemies == 0 {
            wave_state.wave_in_progress = false;
            next_state.set(GameState::CardSelect);
        }
    } else {
        // Wave inactive: start next wave (Wave 1 on initial start, or Wave N after returning from CardSelect)
        let next_wave_num = wave_state.current_wave + 1;
        if let Some(next_wave_def) = generate_wave_definition(next_wave_num) {
            wave_state.start_wave(next_wave_def);
        }
    }
}
