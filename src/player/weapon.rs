use bevy::prelude::*;

use crate::splitter_core::turret::TurretTarget;
use crate::splitter_core::weapon::{ProjectileState, Weapon, WeaponKind, WeaponTrigger};

use bevy::window::PrimaryWindow;

// Marker components to route inputs to the correct barrel
#[derive(Component)]
pub struct MainWeapon;

#[derive(Component)]
pub struct CoaxialWeapon;

#[derive(Component)]
pub struct PlayerTurret;

// --------------------------------------------------------
// NEW: Map mouse clicks to the generic trigger components
// --------------------------------------------------------
pub fn player_weapon_input(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut q_main: Query<&mut WeaponTrigger, (With<MainWeapon>, Without<CoaxialWeapon>)>,
    mut q_coax: Query<&mut WeaponTrigger, (With<CoaxialWeapon>, Without<MainWeapon>)>,
) {
    let is_left_clicking = mouse_input.pressed(MouseButton::Left);
    let is_right_clicking = mouse_input.pressed(MouseButton::Right);

    for mut trigger in q_main.iter_mut() {
        trigger.is_firing = is_left_clicking; // Routes to the rotating turret[cite: 2]
    }

    for mut trigger in q_coax.iter_mut() {
        trigger.is_firing = is_right_clicking; // Routes to the fixed hull weapon[cite: 2]
    }
}

pub fn player_mouse_aiming(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut q_turret_target: Query<&mut TurretTarget, With<PlayerTurret>>,
) {
    // Fixes E0599: iter().next() safely returns an Option, bypassing the missing get_single()
    let Some(window) = q_windows.iter().next() else {
        return;
    };
    let Some((camera, camera_transform)) = q_camera.iter().next() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
        for mut target in q_turret_target.iter_mut() {
            target.world_pos = Some(world_position);
        }
    }
}

pub fn player_reload_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Weapon>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        for mut weapon in query.iter_mut() {
            if let WeaponKind::Projectile {
                current_ammo,
                max_ammo,
                reload_delay,
                state,
                ..
            } = &mut weapon.kind
            {
                // Only start a reload if magazine is below capacity and not already reloading
                if *current_ammo < *max_ammo {
                    if let ProjectileState::Reloading(_) = state {
                        // Already reloading, do nothing
                    } else {
                        *state = ProjectileState::Reloading(Timer::from_seconds(
                            *reload_delay,
                            TimerMode::Once,
                        ));
                    }
                }
            }
        }
    }
}
