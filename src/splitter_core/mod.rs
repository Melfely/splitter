pub mod hit_particle;
pub mod laser;
pub mod projectile;
pub mod turret;
pub mod weapon;

use crate::splitter_core::hit_particle::update_hit_particles;
use crate::splitter_core::projectile::{handle_projectile_collisions, update_projectile_movement};
use crate::splitter_core::turret::update_turret_aiming;
use crate::splitter_core::weapon::update_weapon_firing;

use bevy::prelude::*;

pub const PLAYER_ATTACK_DISTANCE: f32 = 5000.0;

/// The core plugin that registers all shared utilities and mechanics.

#[derive(Component)]
pub struct Lifetime(pub Timer);
pub fn despawn_expired_lifetimes(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.0.tick(time.delta());
        if lifetime.0.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub struct SplitterCorePlugin;

impl Plugin for SplitterCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_turret_aiming)
            .add_systems(Update, update_weapon_firing)
            .add_systems(
                Update,
                (
                    update_projectile_movement,
                    handle_projectile_collisions,
                    despawn_expired_lifetimes,
                    update_hit_particles,
                ),
            );

        // As you build out more shared utilities (like physics mass calculations or generic health components),
        // register their systems and events here.
    }
}
