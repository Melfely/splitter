use bevy::prelude::*;
use std::collections::HashSet;

use crate::splitter_core::laser::LaserPattern;
use crate::splitter_core::projectile::Projectile;

/// A simple boolean toggle. The Player mouse, Enemy AI, or D.A.V.E. just flip this to true/false.
#[derive(Component, Default)]
pub struct WeaponTrigger {
    pub is_firing: bool,
}

#[derive(Component)]
pub struct Weapon {
    pub kind: WeaponKind,
}

pub enum WeaponKind {
    Projectile {
        mass: f32,                  // Counters enemy bulk[cite: 2]
        speed: f32,                 // Counters enemy hardness[cite: 2]
        can_pierce: bool,           // Passed to projectile to allow over-piercing[cite: 2]
        aoe_max_range: Option<f32>, // Passed to projectile to trigger explosion[cite: 2]
        current_ammo: u32,
        max_ammo: u32, // Defines magazine size (1 for single-shot)[cite: 2]
        fire_timer: Timer,
        reload_timer: Timer,
        // Visual assets defined per-weapon
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
                fire_timer,
                reload_timer,
                current_ammo,
                max_ammo,
                ..
            } => {
                fire_timer.tick(time.delta());

                if *current_ammo == 0 {
                    reload_timer.tick(time.delta());
                    if reload_timer.just_finished() {
                        *current_ammo = *max_ammo;
                        reload_timer.reset(); // Reset the clock so future reload cycles can finish!
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
                    current_ammo,
                    fire_timer,
                    mass,
                    speed,
                    can_pierce,
                    aoe_max_range,
                    mesh,
                    material,
                    ..
                } => {
                    if *current_ammo > 0 && fire_timer.is_finished() {
                        *current_ammo -= 1;
                        fire_timer.reset();

                        commands.spawn((
                            Projectile {
                                mass: *mass,
                                speed: *speed,
                                hit_entities: HashSet::new(),
                                can_pierce: *can_pierce,
                                aoe_max_range: *aoe_max_range,
                                distance_traveled: 0.0,
                            },
                            // Visuals attached directly from weapon configuration
                            Mesh2d(mesh.clone()),
                            MeshMaterial2d(material.clone()),
                            Transform::from_xyz(spawn_pos.x, spawn_pos.y, 0.0)
                                .with_rotation(global_transform.compute_transform().rotation),
                            GlobalTransform::default(),
                        ));
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

                        // TODO: Pass `points` to physics system to check segment intersections with enemy limbs[cite: 1]
                    }
                }
            }
        }
    }
}
