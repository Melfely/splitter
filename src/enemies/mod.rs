use crate::GameplaySet;
use bevy::prelude::*;

pub mod ai;
pub mod collisions;
pub mod definitions;
pub mod spawner;
pub mod types;

use crate::enemies::collisions::handle_enemy_body_collisions;
use ai::update_enemy_ai;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_enemy_ai, handle_enemy_body_collisions).in_set(GameplaySet),
        );

        // Future spawning systems for bosses, D.A.V.E., and standard enemies will be registered here[cite: 2].
    }
}
