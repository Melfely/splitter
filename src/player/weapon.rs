use bevy::prelude::*;
use std::collections::HashSet;

use crate::splitter_core::turret::TurretTarget;
use crate::splitter_core::weapon::{ProjectileState, Weapon, WeaponKind, WeaponTrigger};

use crate::physics::collisions::ray_circle_intersection;
use crate::physics::definitions::{CELL_SIZE, Collider, CollisionLayer, SpatialGrid};
use crate::player::PLAYER_ATTACK_DISTANCE;

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

pub fn render_turret_aim_indicator(
    grid: Res<SpatialGrid>,
    turret_query: Query<&GlobalTransform, With<PlayerTurret>>,
    collider_query: Query<(Entity, &GlobalTransform, &Collider)>,
    mut gizmos: Gizmos,
) {
    let Ok(turret_transform) = turret_query.single() else {
        return;
    };

    let origin = turret_transform.translation().truncate();
    let direction = turret_transform.up().truncate();

    let mut closest_hit_dist = PLAYER_ATTACK_DISTANCE;
    let mut hit_entity: Option<Entity> = None;
    let mut visited_entities = HashSet::new();

    let step_size = CELL_SIZE;
    let steps = (PLAYER_ATTACK_DISTANCE / step_size).ceil() as usize;

    for i in 0..=steps {
        let sample_point = origin + direction * (i as f32 * step_size);
        let cell_x = (sample_point.x / CELL_SIZE).floor() as i32;
        let cell_y = (sample_point.y / CELL_SIZE).floor() as i32;

        if let Some(entities) = grid.cells.get(&(cell_x, cell_y)) {
            for &entity in entities {
                if !visited_entities.insert(entity) {
                    continue;
                }

                let Ok((_, target_transform, collider)) = collider_query.get(entity) else {
                    continue;
                };

                if collider.layer != CollisionLayer::EnemyMainBody
                    && collider.layer != CollisionLayer::EnemyLimb
                {
                    continue;
                }

                let target_pos = target_transform.translation().truncate();
                if let Some(dist) =
                    ray_circle_intersection(origin, direction, target_pos, collider.radius)
                {
                    if dist < closest_hit_dist {
                        closest_hit_dist = dist;
                        hit_entity = Some(entity);
                    }
                }
            }
        }
    }

    let impact_pos = origin + direction * closest_hit_dist;
    let dark_blue = Color::srgb(0.0, 0.2, 0.85);

    // Render ONLY the Dark Blue Crosshair at the impact location
    let crosshair_size = 8.0;

    // Horizontal line
    gizmos.line_2d(
        impact_pos - Vec2::X * crosshair_size,
        impact_pos + Vec2::X * crosshair_size,
        dark_blue,
    );
    // Vertical line
    gizmos.line_2d(
        impact_pos - Vec2::Y * crosshair_size,
        impact_pos + Vec2::Y * crosshair_size,
        dark_blue,
    );

    // Circle lock-on ring when pointing at an enemy
    if hit_entity.is_some() {
        gizmos.circle_2d(impact_pos, 12.0, dark_blue);
    }
}
