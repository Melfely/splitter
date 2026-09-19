use bevy::prelude::*;

pub mod ai;
pub mod definitions;
pub mod spawner;

use ai::update_enemy_ai;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_enemy_ai);

        // Future spawning systems for bosses, D.A.V.E., and standard enemies will be registered here[cite: 2].
    }
}
