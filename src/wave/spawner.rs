use super::definitions::WaveState;
use crate::camera::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::enemies::definitions::EnemyMainBody;
use bevy::prelude::*;
use rand::RngExt;

/// Distance off-screen where enemies will spawn.
const SPAWN_MARGIN: f32 = 60.0;

/// Returns a random 2D coordinate positioned just outside the arena boundary.
pub fn random_offscreen_position() -> Vec2 {
    let mut rng = rand::rng();
    let half_w = ARENA_WIDTH / 2.0;
    let half_h = ARENA_HEIGHT / 2.0;

    // Pick a random edge: 0 = Top, 1 = Bottom, 2 = Left, 3 = Right
    match rng.random_range(0..4) {
        0 => Vec2::new(
            rng.random_range((-half_w - SPAWN_MARGIN)..(half_w + SPAWN_MARGIN)),
            half_h + SPAWN_MARGIN,
        ),
        1 => Vec2::new(
            rng.random_range((-half_w - SPAWN_MARGIN)..(half_w + SPAWN_MARGIN)),
            -half_h - SPAWN_MARGIN,
        ),
        2 => Vec2::new(
            -half_w - SPAWN_MARGIN,
            rng.random_range((-half_h - SPAWN_MARGIN)..(half_h + SPAWN_MARGIN)),
        ),
        _ => Vec2::new(
            half_w + SPAWN_MARGIN,
            rng.random_range((-half_h - SPAWN_MARGIN)..(half_h + SPAWN_MARGIN)),
        ),
    }
}

/// Increments global game timer and processes pending queue spawns.
pub fn update_wave_spawner(
    time: Res<Time>,
    mut wave_state: ResMut<WaveState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !wave_state.wave_in_progress {
        return;
    }

    wave_state.game_timer += time.delta_secs();

    if wave_state.pending_spawns.is_empty() {
        if wave_state.current_enemies == 0 {
            wave_state.wave_in_progress = false;
        }
        return;
    }

    wave_state.spawn_timer.tick(time.delta());
    if wave_state.spawn_timer.just_finished() {
        let to_spawn = wave_state.batch_size.min(wave_state.pending_spawns.len());

        for _ in 0..to_spawn {
            if let Some(spawner_fn) = wave_state.pending_spawns.pop() {
                let spawn_pos = random_offscreen_position();
                spawner_fn(&mut commands, &mut meshes, &mut materials, spawn_pos);
            }
        }
    }
}

/// Keeps live count accurate and tracks total kills when main body entities are removed/despawned.
pub fn track_enemy_stats(
    mut wave_state: ResMut<WaveState>,
    enemy_query: Query<&EnemyMainBody>,
    mut removed_enemies: RemovedComponents<EnemyMainBody>,
) {
    let kills = removed_enemies.read().count() as u64;
    if kills > 0 {
        wave_state.total_enemies_killed += kills;
    }

    wave_state.current_enemies = enemy_query.iter().count();
}
