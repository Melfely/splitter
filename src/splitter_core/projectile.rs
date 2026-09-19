use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Component)]
pub struct Projectile {
    /// Counters enemy bulk defense. If lower than bulk, damage is reduced.
    pub mass: f32,

    /// Counters enemy hardness defense. If lower than hardness, damage is reduced[cite: 1].
    pub speed: f32,

    /// Tracks entities already hit. If a limb is hit first, the main body is added here
    /// to make it immune to this specific projectile, unless the limb is destroyed[cite: 1].
    pub hit_entities: HashSet<Entity>,

    /// True if this is a piercing projectile capable of over-piercing limbs[cite: 1].
    pub can_pierce: bool,

    /// AOE projectiles uniquely have a max range[cite: 1].
    pub aoe_max_range: Option<f32>,

    /// Tracks how far the projectile has traveled to trigger AOE explosions.
    pub distance_traveled: f32,
}

pub fn update_projectile_movement(
    time: Res<Time>,
    mut commands: Commands,
    mut projectiles: Query<(Entity, &mut Transform, &mut Projectile)>,
) {
    for (entity, mut transform, mut projectile) in projectiles.iter_mut() {
        // Calculate movement step based on the speed stat
        // Assuming sprites point UP (Y-axis) by default
        let step = transform.up() * projectile.speed * time.delta_secs();

        transform.translation += step;
        projectile.distance_traveled += step.length();

        // Handle unique AOE max range detonation[cite: 1]
        if let Some(max_range) = projectile.aoe_max_range {
            if projectile.distance_traveled >= max_range {
                // TODO: Spawn multiple shrapnel projectiles that deal damage[cite: 1]
                commands.entity(entity).despawn();
            }
        }
    }
}
