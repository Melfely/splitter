use super::definitions::{WaveDefinition, WaveEnemyGroup, WaveState};
use crate::enemies::types::{spawn_fast_red_charger, spawn_heavy_yellow_tank};
use bevy::prelude::*;

/// Generates a `WaveDefinition` for waves 1 through 20 based on wave scaling rules.
/// Returns `None` if the requested wave number exceeds wave 20.
pub fn generate_wave_definition(wave_number: u32) -> Option<WaveDefinition> {
    if wave_number < 1 || wave_number > 20 {
        return None;
    }

    // Fast red chargers start at 10 and increase by 10 per wave number
    let red_charger_count = (wave_number * 10) as usize;

    // Batch size starts at 5 for wave 1 and ramps up by +1 per wave number
    let batch_size = (5 + (wave_number - 1)) as usize;

    let mut groups = vec![WaveEnemyGroup {
        spawner: spawn_fast_red_charger,
        count: red_charger_count,
    }];

    // Waves 11 through 20 include exactly 1 heavy yellow tank
    if (11..=20).contains(&wave_number) {
        groups.push(WaveEnemyGroup {
            spawner: spawn_heavy_yellow_tank,
            count: 1,
        });
    }

    Some(WaveDefinition {
        wave_number,
        groups,
        spawn_interval: 0.25,
        batch_size,
    })
}

/// System that automatically advances to and starts the next wave when no wave is currently active.
pub fn auto_wave_advancement_system(mut wave_state: ResMut<WaveState>) {
    if !wave_state.wave_in_progress {
        let next_wave_num = wave_state.current_wave + 1;
        if let Some(next_wave_def) = generate_wave_definition(next_wave_num) {
            wave_state.start_wave(next_wave_def);
        }
    }
}
