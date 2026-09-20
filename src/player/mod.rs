use crate::GameplaySet;
use crate::physics::definitions::{Collider, CollisionLayer};
use crate::player::collisions::handle_player_collisions;
use crate::player::definitions::{
    MountPoint, MountedWeapon, Player, PlayerBlink, PlayerLoadout, PlayerPhysics, PlayerShield,
    PlayerTurret, Shield,
};
use crate::player::health::player_health_system;
use crate::player::mount::attach_mounted_weapon;
use crate::player::movement::{handle_out_of_bounds_damage, update_player_movement};
use crate::player::shield::{sync_shield_collider_and_visibility, update_shield_system};
use crate::player::weapon::{
    player_mouse_aiming, player_reload_input, player_weapon_input, render_turret_aim_indicator,
};
use crate::player::weapons::{create_autocannon, create_heavy_cannon};

use crate::splitter_core::PLAYER_ATTACK_DISTANCE;
use crate::splitter_core::projectile::Durability;
use crate::splitter_core::turret::{Turret, TurretTarget};

pub mod collisions;
pub mod definitions;
pub mod health;
pub mod mount;
pub mod movement;
pub mod shield;
pub mod weapon;
pub mod weapons;

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
    loadout: Option<Res<PlayerLoadout>>,
) {
    let hull_color = materials.add(Color::srgb(0.3, 0.3, 0.3));
    let turret_color = materials.add(Color::srgb(0.5, 0.5, 0.5));
    let weapon_barrel_color = materials.add(Color::srgb(0.1, 0.1, 0.1));
    let shield_material = materials.add(Color::srgba(0.2, 0.7, 1.0, 0.05));

    // Fallback containers used if no PlayerLoadout resource exists in the world
    let fallback_coaxial;
    let fallback_turret;

    let (coaxial_mounts, turret_mounts) = if let Some(ref loadout) = loadout {
        (&loadout.coaxial_mounts, &loadout.turret_mounts)
    } else {
        // Construct standard default weapons if loadout resource was not initialized
        fallback_coaxial = vec![MountedWeapon {
            weapon: create_autocannon(&mut meshes, &mut materials),
            mount_point: MountPoint::Coaxial {
                // Z = 0.05 places coaxial barrels above Hull (0.0) but under Turret (0.1)
                offset: Vec3::new(20.0, 40.0, 0.05),
            },
            mesh: meshes.add(Rectangle::new(10.0, 40.0)),
            material: weapon_barrel_color.clone(),
        }];

        fallback_turret = vec![MountedWeapon {
            weapon: create_heavy_cannon(&mut meshes, &mut materials),
            mount_point: MountPoint::Turret {
                // Local Z = 0.05 places turret barrels above Turret mesh (0.1)
                offset: Vec3::new(0.0, 30.0, 0.05),
            },
            mesh: meshes.add(Rectangle::new(12.0, 50.0)),
            material: weapon_barrel_color.clone(),
        }];

        (&fallback_coaxial, &fallback_turret)
    };

    commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(60.0, 80.0))),
            MeshMaterial2d(hull_color),
            Transform::from_xyz(0.0, 0.0, 0.0), // Hull Base: Z = 0.0
            PlayerBlink::default(),
            Player {},
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
            // Shield Child Entity (Z = 0.2, renders above everything)
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

            // Safely attach coaxial weapons (Z = 0.05)
            for mounted_weapon in coaxial_mounts.iter().cloned() {
                attach_mounted_weapon(parent, mounted_weapon);
            }

            // Main Turret (Z = 0.1, renders above Hull and Coaxial weapons)
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
                    // Safely attach turret weapons (Local Z = 0.05, renders above Turret Base)
                    for mounted_weapon in turret_mounts.iter().cloned() {
                        attach_mounted_weapon(turret, mounted_weapon);
                    }
                });
        });
}
