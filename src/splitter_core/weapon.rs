use bevy::prelude::*;
use std::collections::HashSet;

use crate::GameState;
use crate::physics::definitions::{Collider, CollisionLayer};
use crate::splitter_core::laser::LaserPattern;
use crate::splitter_core::projectile::{AoeSplitConfig, Projectile, calculate_lifetime_from_speed};

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
    /// Weapon is ready to fire immediately, holding its shot delay accumulator.
    Ready { fire_accumulator: f32 },
    /// Full reload: Reloading an entire magazine or resetting a single-shot cannon.
    Reloading(Timer),
}

impl Default for ProjectileState {
    fn default() -> Self {
        Self::Ready {
            fire_accumulator: 0.0,
        }
    }
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
        pellets_per_shot: u32, // Multi-shot count (default: 1)
        spread_angle: f32,     // Multi-shot fan angle in radians
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
    mut gizmos: Gizmos,
    mut weapons: Query<(&GlobalTransform, &mut Weapon, &WeaponTrigger)>,
) {
    let delta_secs = time.delta_secs();

    for (global_transform, mut weapon, trigger) in weapons.iter_mut() {
        // 1. Process passive weapon mechanics (reloads & cooling)
        match &mut weapon.kind {
            WeaponKind::Projectile {
                current_ammo,
                max_ammo,
                reload_delay,
                fire_delay,
                state,
                ..
            } => match state {
                ProjectileState::Ready { fire_accumulator } => {
                    if *current_ammo == 0 {
                        *state = ProjectileState::Reloading(Timer::from_seconds(
                            *reload_delay,
                            TimerMode::Once,
                        ));
                    } else if !trigger.is_firing {
                        *fire_accumulator = (*fire_accumulator + delta_secs).min(*fire_delay);
                    }
                }
                ProjectileState::Reloading(timer) => {
                    timer.tick(time.delta());
                    if timer.just_finished() {
                        *current_ammo = *max_ammo;
                        *state = ProjectileState::Ready {
                            fire_accumulator: *fire_delay,
                        };
                    }
                }
            },
            WeaponKind::Laser {
                recharge_timer,
                current_heat,
                cooling_rate,
                is_recharging,
                ..
            } => {
                if !trigger.is_firing || *is_recharging {
                    *current_heat = (*current_heat - *cooling_rate * delta_secs).max(0.0);
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
            let forward_dir = (global_transform.compute_transform().rotation * Vec3::Y)
                .truncate()
                .normalize_or_zero();
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
                    pellets_per_shot,
                    spread_angle,
                    state,
                    mesh,
                    material,
                    ..
                } => {
                    if let ProjectileState::Ready { fire_accumulator } = state {
                        *fire_accumulator += delta_secs;
                        let effective_delay = fire_delay.max(0.0001);

                        while *fire_accumulator >= effective_delay && *current_ammo > 0 {
                            *fire_accumulator -= effective_delay;
                            *current_ammo -= 1;

                            // --- Multi-shot Spread Calculation ---
                            let pellets = (*pellets_per_shot).max(1);
                            let base_rotation = global_transform.compute_transform().rotation;

                            // Convert spread_angle from degrees to radians here
                            let spread_radians = spread_angle.to_radians();

                            let angle_step = if pellets > 1 {
                                spread_radians / (pellets - 1) as f32
                            } else {
                                0.0
                            };
                            let start_angle = if pellets > 1 {
                                -spread_radians / 2.0
                            } else {
                                0.0
                            };

                            for i in 0..pellets {
                                let offset_angle = start_angle + (i as f32) * angle_step;
                                let pellet_rotation =
                                    base_rotation * Quat::from_rotation_z(offset_angle);

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
                                    lifetime,
                                    Mesh2d(mesh.clone()),
                                    MeshMaterial2d(material.clone()),
                                    Transform::from_xyz(spawn_pos.x, spawn_pos.y, 0.0)
                                        .with_rotation(pellet_rotation),
                                    GlobalTransform::default(),
                                    DespawnOnEnter(GameState::MainMenu),
                                ));
                            }

                            if *current_ammo == 0 {
                                *state = ProjectileState::Reloading(Timer::from_seconds(
                                    *reload_delay,
                                    TimerMode::Once,
                                ));
                                break;
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
                        *current_heat += 20.0 * delta_secs;

                        if *current_heat >= *max_heat {
                            *is_recharging = true;
                        }

                        let points =
                            pattern.generate_points(spawn_pos, forward_dir, time.elapsed_secs());

                        gizmos.linestrip_2d(points, *color);
                    }
                }
            }
        }
    }
}
