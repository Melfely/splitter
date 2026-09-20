use super::definitions::{
    ARENA_HALF_HEIGHT, ARENA_HALF_WIDTH, OUT_OF_BOUNDS_DAMAGE_PER_SEC, PlayerPhysics,
};
use crate::enemies::definitions::EnemyMainBody;
use crate::player::{Player, PlayerBlink};
use crate::splitter_core::projectile::Durability;
use bevy::prelude::*;

pub fn update_player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        &mut Transform,
        &mut PlayerPhysics,
        &Player,
        &mut PlayerBlink,
    )>,
    mut enemy_query: Query<&mut Transform, (With<EnemyMainBody>, Without<Player>)>,
) {
    let delta_time = time.delta_secs();

    for (mut transform, mut physics, _player, mut blink) in query.iter_mut() {
        // Cache physics parameters locally
        let accel = physics.acceleration;
        let turn_accel = physics.turn_acceleration;
        let blink_impulse = physics.blink_impulse;
        let forward_drag = physics.forward_drag;
        let lateral_drag = physics.lateral_drag;
        let angular_drag = physics.angular_drag;
        let max_speed = physics.max_speed;
        let max_turn_speed = physics.max_turn_speed;
        let blink_distance = physics.blink_distance;

        let forward_dir = transform.local_y().truncate();
        let right_dir = transform.local_x().truncate();

        // --- 1. WASD Drive Input ---
        let mut drive_input = 0.0;
        let mut turn_input = 0.0;

        if keyboard_input.pressed(KeyCode::KeyW) {
            drive_input += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            drive_input -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            turn_input += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            turn_input -= 1.0;
        }

        let current_forward_speed = physics.velocity.dot(forward_dir);
        if drive_input > 0.0 && current_forward_speed < max_speed {
            physics.velocity += forward_dir * drive_input * accel * delta_time;
        } else if drive_input < 0.0 && current_forward_speed > -max_speed {
            physics.velocity += forward_dir * drive_input * accel * delta_time;
        }

        if turn_input != 0.0 {
            physics.angular_velocity = (physics.angular_velocity
                + turn_input * turn_accel * delta_time)
                .clamp(-max_turn_speed, max_turn_speed);
        }

        // Helper closure for landing clearance
        let clear_landing_zone = |landing_pos: Vec2,
                                  enemy_query: &mut Query<
            &mut Transform,
            (With<EnemyMainBody>, Without<Player>),
        >| {
            let clearance_radius = 95.0; // Force-clears enemies in landing area
            for mut enemy_tf in enemy_query.iter_mut() {
                let enemy_pos = enemy_tf.translation.truncate();
                let delta = enemy_pos - landing_pos;
                let dist = delta.length();

                if dist < clearance_radius {
                    let push_dir = if dist > 0.001 { delta / dist } else { Vec2::Y };
                    let push_dist = (clearance_radius - dist) + 35.0;
                    enemy_tf.translation += (push_dir * push_dist).extend(0.0);
                }
            }
        };

        // --- 2. Q/E Strafe Blink Impulse & Landing Shockwave ---
        if keyboard_input.just_pressed(KeyCode::KeyQ) && blink.q_cooldown.is_finished() {
            let left_dir = -right_dir;
            transform.translation += (left_dir * blink_distance).extend(0.0);
            physics.velocity += left_dir * blink_impulse;
            blink.q_cooldown.reset();

            clear_landing_zone(transform.translation.truncate(), &mut enemy_query);
        }

        if keyboard_input.just_pressed(KeyCode::KeyE) && blink.e_cooldown.is_finished() {
            transform.translation += (right_dir * blink_distance).extend(0.0);
            physics.velocity += right_dir * blink_impulse;
            blink.e_cooldown.reset();

            clear_landing_zone(transform.translation.truncate(), &mut enemy_query);
        }

        // --- 3. Friction & Deceleration ---
        let current_forward = physics.velocity.dot(forward_dir);
        let current_lateral = physics.velocity.dot(right_dir);

        let forward_vec = forward_dir * current_forward;
        let lateral_vec = right_dir * current_lateral;

        let damped_forward = forward_vec * (-forward_drag * delta_time).exp();
        let damped_lateral = lateral_vec * (-lateral_drag * delta_time).exp();

        physics.velocity = damped_forward + damped_lateral;
        physics.angular_velocity *= (-angular_drag * delta_time).exp();

        // --- 4. Apply Position & Rotation ---
        transform.rotate_z(physics.angular_velocity * delta_time);
        transform.translation += (physics.velocity * delta_time).extend(0.0);
    }
}

pub fn handle_out_of_bounds_damage(
    time: Res<Time>,
    mut player_query: Query<(&Transform, &mut Durability), With<Player>>,
) {
    let delta_time = time.delta_secs();

    for (transform, mut durability) in player_query.iter_mut() {
        let pos = transform.translation.truncate();

        if pos.x.abs() > ARENA_HALF_WIDTH || pos.y.abs() > ARENA_HALF_HEIGHT {
            durability.hp = (durability.hp - OUT_OF_BOUNDS_DAMAGE_PER_SEC * delta_time).max(0.0);
        }
    }
}
