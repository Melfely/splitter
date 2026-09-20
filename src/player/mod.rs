use crate::GameplaySet;
use crate::physics::definitions::{Collider, CollisionLayer};
use crate::player::collisions::handle_player_collisions;
use crate::player::definitions::{Player, PlayerBlink, PlayerPhysics};
use crate::player::health::player_health_system;
use crate::player::movement::{handle_out_of_bounds_damage, update_player_movement};
use crate::player::shield::{
    PlayerShield, Shield, sync_shield_collider_and_visibility, update_shield_system,
};
use crate::player::weapon::{
    CoaxialWeapon, MainWeapon, PlayerTurret, player_mouse_aiming, player_reload_input,
    player_weapon_input, render_turret_aim_indicator,
};
use crate::splitter_core::PLAYER_ATTACK_DISTANCE;
use crate::splitter_core::laser::LaserPattern;
use crate::splitter_core::projectile::Durability;
use crate::splitter_core::turret::{Turret, TurretTarget};
use crate::splitter_core::weapon::{ProjectileState, Weapon, WeaponKind, WeaponTrigger};

pub mod collisions;
pub mod definitions;
pub mod health;
pub mod movement;
pub mod shield;
pub mod weapon;

use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnTransition {
                exited: GameState::MainMenu,
                entered: GameState::InGame,
            },
            spawn_player,
        )
        .add_systems(
            Update,
            (
                player_mouse_aiming,
                player_weapon_input,
                player_reload_input,
                render_turret_aim_indicator,
                update_shield_system,
                sync_shield_collider_and_visibility,
                player_health_system,
                handle_player_collisions,
                handle_out_of_bounds_damage,
                update_player_movement,
            )
                .in_set(GameplaySet),
        );
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let hull_color = materials.add(Color::srgb(0.3, 0.3, 0.3));
    let turret_color = materials.add(Color::srgb(0.5, 0.5, 0.5));
    let weapon_color = materials.add(Color::srgb(0.1, 0.1, 0.1));

    // Transparent Light Blue material (Alpha = 0.05)
    let shield_material = materials.add(Color::srgba(0.2, 0.7, 1.0, 0.05));

    commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(60.0, 80.0))),
            MeshMaterial2d(hull_color),
            Transform::from_xyz(0.0, 0.0, 0.0),
            PlayerBlink::default(),
            Player {},
            // Permanent Hull Health
            Durability {
                hp: 10.0,
                max_hp: 10.0,
                hardness: 10.0,
                bulk: 20.0,
            },
            DespawnOnEnter(GameState::MainMenu),
            PlayerPhysics::default(),
        ))
        .with_children(|parent| {
            // Shield Child Entity: Expanded Ellipse (65.0x85.0)
            parent.spawn((
                PlayerShield,
                Shield::default(),
                Durability {
                    hp: 50.0,
                    max_hp: 50.0,
                    hardness: 2.0,
                    bulk: 5.0,
                },
                Mesh2d(meshes.add(Ellipse::new(65.0, 85.0))),
                MeshMaterial2d(shield_material),
                Transform::from_xyz(0.0, 0.0, 0.2),
                Collider {
                    radius: 85.0,
                    layer: CollisionLayer::Player,
                },
            ));

            // Coaxial Weapon
            parent.spawn((
                Mesh2d(meshes.add(Rectangle::new(10.0, 40.0))),
                MeshMaterial2d(weapon_color.clone()),
                Transform::from_xyz(20.0, 40.0, -0.1),
                CoaxialWeapon,
                WeaponTrigger::default(),
                Weapon {
                    kind: WeaponKind::Laser {
                        intensity: 15.0,
                        focus: 0.8,
                        current_heat: 0.0,
                        max_heat: 100.0,
                        heating_rate: 80.0,
                        cooling_rate: 40.0,
                        recharge_timer: Timer::from_seconds(2.0, TimerMode::Once),
                        is_recharging: false,
                        pattern: LaserPattern::Lightning {
                            length: PLAYER_ATTACK_DISTANCE,
                            segment_length: 50.0,
                            jitter: 15.0,
                        },
                        color: Color::srgb(0.0, 0.8, 1.0),
                    },
                },
            ));

            // Main Turret
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
                    turret.spawn((
                        Mesh2d(meshes.add(Rectangle::new(12.0, 50.0))),
                        MeshMaterial2d(weapon_color),
                        Transform::from_xyz(0.0, 30.0, -0.1),
                        MainWeapon,
                        WeaponTrigger::default(),
                        Weapon {
                            kind: WeaponKind::Projectile {
                                mass: 25.0,
                                speed: 300.0,
                                radius: 8.0,
                                layer: CollisionLayer::PlayerProjectile,
                                can_pierce: true,
                                aoe_max_range: None,
                                current_ammo: 1,
                                max_ammo: 1,
                                fire_delay: 0.0,
                                reload_delay: 1.25,
                                state: ProjectileState::Ready,
                                mesh: meshes.add(Rectangle::new(8.0, 16.0)),
                                material: materials.add(Color::srgb(0.9, 0.8, 0.2)),
                            },
                        },
                    ));
                });
        });
}
