use bevy::prelude::*;

pub mod ai;
pub mod collisions;
pub mod definitions;
pub mod spawner;
pub mod types;

use crate::enemies::collisions::handle_enemy_body_collisions;
use crate::enemies::types::{spawn_fast_red_charger, spawn_heavy_yellow_tank};
use ai::update_enemy_ai;

use crate::camera::{ARENA_HEIGHT, ARENA_WIDTH};

/// System to populate the entire arena screen with 10,000 fast red enemies in a uniform grid layout.
pub fn setup_initial_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let total_enemies = 200;
    let grid_cols = 100; // 100 x 100 grid = 1,000 enemies
    let grid_rows = total_enemies / grid_cols;

    let half_width = ARENA_WIDTH / 2.0;
    let half_height = ARENA_HEIGHT / 2.0;

    let step_x = ARENA_WIDTH / grid_cols as f32;
    let step_y = ARENA_HEIGHT / grid_rows as f32;

    for i in 0..total_enemies {
        let col = i % grid_cols;
        let row = i / grid_cols;

        // Offset grid cells so enemies are evenly distributed within [-half, +half]
        let x = -half_width + (col as f32 + 0.5) * step_x;
        let y = -half_height + (row as f32 + 0.5) * step_y;

        spawn_fast_red_charger(&mut commands, &mut meshes, &mut materials, Vec2::new(x, y));
    }

    // Spawn a heavy yellow tank at the center
    spawn_heavy_yellow_tank(&mut commands, &mut meshes, &mut materials, Vec2::ZERO);
}

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_enemy_ai, handle_enemy_body_collisions))
            .add_systems(Startup, setup_initial_enemies);

        // Future spawning systems for bosses, D.A.V.E., and standard enemies will be registered here[cite: 2].
    }
}
