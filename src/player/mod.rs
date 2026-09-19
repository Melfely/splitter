use crate::splitter_core::laser::LaserPattern;
use crate::splitter_core::turret::{Turret, TurretTarget};
use crate::splitter_core::weapon::{ProjectileState, Weapon, WeaponKind, WeaponTrigger};

use crate::splitter_core::PLAYER_ATTACK_DISTANCE;

use crate::player::movement::{PlayerBlink, player_movement};
use crate::player::weapon::{
    CoaxialWeapon, MainWeapon, PlayerTurret, player_mouse_aiming, player_reload_input,
    player_weapon_input,
};

use crate::physics::definitions::{Collider, CollisionLayer};

pub mod movement;
pub mod weapon;

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (
                player_movement,
                player_mouse_aiming,
                player_weapon_input,
                player_reload_input,
            ),
        );
    }
}

#[derive(Component)]
pub struct Player {
    pub move_speed: f32,
    pub turn_speed: f32,
    pub blink_distance: f32,
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let hull_color = materials.add(Color::srgb(0.3, 0.3, 0.3));
    let turret_color = materials.add(Color::srgb(0.5, 0.5, 0.5));
    let weapon_color = materials.add(Color::srgb(0.1, 0.1, 0.1));

    commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(60.0, 80.0))),
            MeshMaterial2d(hull_color),
            Transform::from_xyz(0.0, 0.0, 0.0),
            PlayerBlink::default(),
            Player {
                move_speed: 150.0,
                turn_speed: 2.5,
                blink_distance: 100.0,
            },
            // Added Collider explicitly to the hull entity
            Collider {
                radius: 40.0, // Tightly bounds the 60x80 rectangle
                layer: CollisionLayer::Player,
            },
        ))
        .with_children(|parent| {
            // Coaxial Weapon (Fixed to front right of the hull)
            parent.spawn((
                Mesh2d(meshes.add(Rectangle::new(10.0, 40.0))),
                MeshMaterial2d(weapon_color.clone()),
                Transform::from_xyz(20.0, 40.0, -0.1),
                CoaxialWeapon, // Marker for Right Click
                WeaponTrigger::default(),
                Weapon {
                    kind: WeaponKind::Laser {
                        intensity: 15.0,
                        focus: 0.8,
                        current_heat: 0.0,
                        max_heat: 100.0,
                        heating_rate: 80.0, // Overheats in ~1.25s of continuous firing
                        cooling_rate: 40.0, // Fully cools in 2.5s from max heat
                        recharge_timer: Timer::from_seconds(2.0, TimerMode::Once),
                        is_recharging: false,
                        pattern: LaserPattern::Lightning {
                            length: PLAYER_ATTACK_DISTANCE, // Always reaches past screen edges[cite: 1]
                            segment_length: 50.0,
                            jitter: 15.0,
                        },
                        color: Color::srgb(0.0, 0.8, 1.0),
                    },
                },
            ));

            // Main Turret (Center of hull) - Aiming components attached here
            parent
                .spawn((
                    Mesh2d(meshes.add(Rectangle::new(40.0, 40.0))),
                    MeshMaterial2d(turret_color),
                    Transform::from_xyz(0.0, 0.0, 0.1),
                    Turret { turn_speed: 10.0 },
                    TurretTarget::default(),
                    PlayerTurret,
                ))
                .with_children(|turret| {
                    // Main Turret Barrel
                    turret.spawn((
                        Mesh2d(meshes.add(Rectangle::new(12.0, 50.0))),
                        MeshMaterial2d(weapon_color),
                        Transform::from_xyz(0.0, 30.0, -0.1),
                        MainWeapon, // Marker for Left Click
                        WeaponTrigger::default(),
                        Weapon {
                            kind: WeaponKind::Projectile {
                                mass: 15.0,
                                speed: 450.0,
                                radius: 4.0, // Matches 4.0 circle mesh radius
                                layer: CollisionLayer::PlayerProjectile, // Correct collision layer
                                can_pierce: false,
                                aoe_max_range: None,
                                current_ammo: 20,
                                max_ammo: 20,
                                fire_delay: 0.1,
                                reload_delay: 2.5,
                                state: ProjectileState::Ready,
                                mesh: meshes.add(Circle::new(4.0)),
                                material: materials.add(Color::srgb(0.9, 0.1, 0.1)),
                            },
                        },
                    ));
                });
        });
}
