use bevy::prelude::*;
use std::collections::HashSet;

use crate::splitter_core::laser::LaserPattern;
use crate::splitter_core::projectile::{AoeSplitConfig, Projectile, calculate_lifetime_from_speed};

use crate::GameState;
use crate::physics::definitions::{Collider, CollisionLayer};

/// A simple boolean toggle. The Player mouse, Enemy AI, or D.A.V.E. just flip this to true/false.
#[derive(Component, Default)]
pub struct WeaponTrigger {
    pub is_firing: bool,
}

#[derive(Component, Clone)]
pub struct Weapon {
    pub kind: WeaponKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ProjectileState {
    /// Weapon is ready to fire immediately.
    Ready,
    /// Intra-reloading: The delay between consecutive shots in the same magazine.
    IntraReloading(Timer),
    /// Full reload: Reloading an entire magazine or resetting a single-shot cannon.
    Reloading(Timer),
}

#[derive(Clone)]
pub enum WeaponKind {
    Projectile {
        mass: f32,
        speed: f32,
        radius: f32,           // Collision radius of the projectile
        layer: CollisionLayer, // PlayerProjectile or EnemyProjectile
        can_pierce: bool,
        aoe_max_range: Option<f32>,
        aoe_split: Option<AoeSplitConfig>,
        current_ammo: u32,
        max_ammo: u32,
        fire_delay: f32,
        reload_delay: f32,
        state: ProjectileState,
        mesh: Handle<Mesh>,
        material: Handle<ColorMaterial>,
    },
    Laser {
        intensity: f32,
        focus: f32,
        current_heat: f32,
        max_heat: f32,
        heating_rate: f32, // Heat added per second while firing
        cooling_rate: f32, // Heat removed per second while idle
        recharge_timer: Timer,
        is_recharging: bool,
        pattern: LaserPattern,
        color: Color,
    },
}

pub fn update_weapon_firing(
    time: Res<Time>,
    mut commands: Commands,
    mut weapons: Query<(&GlobalTransform, &mut Weapon, &WeaponTrigger)>,
) {
    for (global_transform, mut weapon, trigger) in weapons.iter_mut() {
        // 1. Process passive weapon mechanics
        match &mut weapon.kind {
            WeaponKind::Projectile {
                current_ammo,
                max_ammo,
                reload_delay,
                state,
                ..
            } => {
                match state {
                    ProjectileState::Ready => {
                        // Auto-trigger reload if out of ammo
                        if *current_ammo == 0 {
                            *state = ProjectileState::Reloading(Timer::from_seconds(
                                *reload_delay,
                                TimerMode::Once,
                            ));
                        }
                    }
                    ProjectileState::IntraReloading(timer) => {
                        timer.tick(time.delta());
                        if timer.just_finished() {
                            *state = ProjectileState::Ready;
                        }
                    }
                    ProjectileState::Reloading(timer) => {
                        timer.tick(time.delta());
                        if timer.just_finished() {
                            *current_ammo = *max_ammo;
                            *state = ProjectileState::Ready;
                        }
                    }
                }
            }
            WeaponKind::Laser {
                recharge_timer,
                current_heat,
                cooling_rate,
                is_recharging,
                ..
            } => {
                // Only cool down when NOT firing, or when locked in recharge
                if !trigger.is_firing || *is_recharging {
                    *current_heat = (*current_heat - *cooling_rate * time.delta_secs()).max(0.0);
                }

                if *is_recharging {
                    recharge_timer.tick(time.delta());
                    if recharge_timer.just_finished() {
                        *is_recharging = false;
                        recharge_timer.reset();
                    }
                }
            }
        }

        // 2. Execute firing
        if trigger.is_firing {
            let spawn_pos = global_transform.translation().truncate();
            let forward_dir = global_transform.up().truncate();

            match &mut weapon.kind {
                WeaponKind::Projectile {
                    mass,
                    speed,
                    radius,
                    layer,
                    can_pierce,
                    aoe_max_range,
                    aoe_split,
                    current_ammo,
                    fire_delay,
                    reload_delay,
                    state,
                    mesh,
                    material,
                    ..
                } => {
                    if let ProjectileState::Ready = state {
                        if *current_ammo > 0 {
                            *current_ammo -= 1;

                            // Calculate dynamic lifetime based on initial speed
                            let lifetime = calculate_lifetime_from_speed(*speed);

                            commands.spawn((
                                Projectile {
                                    mass: *mass,
                                    speed: *speed,
                                    initial_speed: *speed,
                                    hit_entities: HashSet::new(),
                                    can_pierce: *can_pierce,
                                    aoe_max_range: *aoe_max_range,
                                    distance_traveled: 0.0,
                                    aoe_split: aoe_split.clone(),
                                },
                                Collider {
                                    radius: *radius,
                                    layer: *layer,
                                },
                                lifetime, // Dynamically computed lifetime attached on spawn
                                Mesh2d(mesh.clone()),
                                MeshMaterial2d(material.clone()),
                                Transform::from_xyz(spawn_pos.x, spawn_pos.y, 0.0)
                                    .with_rotation(global_transform.compute_transform().rotation),
                                GlobalTransform::default(),
                                DespawnOnEnter(GameState::MainMenu),
                            ));

                            if *current_ammo == 0 {
                                *state = ProjectileState::Reloading(Timer::from_seconds(
                                    *reload_delay,
                                    TimerMode::Once,
                                ));
                            } else {
                                *state = ProjectileState::IntraReloading(Timer::from_seconds(
                                    *fire_delay,
                                    TimerMode::Once,
                                ));
                            }
                        }
                    }
                }
                WeaponKind::Laser {
                    is_recharging,
                    current_heat,
                    max_heat,
                    pattern,
                    color,
                    ..
                } => {
                    if !*is_recharging {
                        // Accumulate heat while holding trigger
                        *current_heat += 20.0 * time.delta_secs();

                        if *current_heat >= *max_heat {
                            *is_recharging = true;
                        }

                        // Generate beam points dynamically along the pattern path
                        let points =
                            pattern.generate_points(spawn_pos, forward_dir, time.elapsed_secs());

                        // Draw continuous pattern visually using Bevy Gizmos
                        gizmo().linestrip_2d(points.clone(), *color);

                        // TODO: Pass `points` to physics system to check segment intersections with enemy limbs
                    }
                }
            }
        }
    }
}
