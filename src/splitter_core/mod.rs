pub mod laser;
pub mod projectile;
pub mod turret;
pub mod weapon;

use crate::splitter_core::projectile::update_projectile_movement;
use crate::splitter_core::turret::update_turret_aiming;
use crate::splitter_core::weapon::update_weapon_firing;

use bevy::prelude::*;

/// The core plugin that registers all shared utilities and mechanics.
pub struct SplitterCorePlugin;

impl Plugin for SplitterCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_turret_aiming)
            .add_systems(Update, update_weapon_firing)
            .add_systems(Update, update_projectile_movement);

        // As you build out more shared utilities (like physics mass calculations or generic health components),
        // register their systems and events here.
    }
}
