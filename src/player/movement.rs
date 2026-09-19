use bevy::prelude::*;

use crate::player::Player;

#[derive(Component)]
pub struct PlayerBlink {
    pub q_cooldown: Timer,
    pub e_cooldown: Timer,
}

impl Default for PlayerBlink {
    fn default() -> Self {
        Self {
            q_cooldown: Timer::from_seconds(1.5, TimerMode::Once),
            e_cooldown: Timer::from_seconds(1.5, TimerMode::Once),
        }
    }
}

pub fn player_movement(
    mut query: Query<(&mut Transform, &Player)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in query.iter_mut() {
        let mut forward_movement = 0.0;
        let mut rotation_movement = 0.0;

        // Forward/Backward (W/S)
        if keyboard_input.pressed(KeyCode::KeyW) {
            forward_movement += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            forward_movement -= 1.0;
        }

        // Steering (A/D)
        if keyboard_input.pressed(KeyCode::KeyA) {
            rotation_movement += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            rotation_movement -= 1.0;
        }

        // Fixed: delta_seconds() is now delta_secs()
        transform.rotate_z(rotation_movement * player.turn_speed * time.delta_secs());

        let up = transform.local_y();
        transform.translation += up * forward_movement * player.move_speed * time.delta_secs();

        // Blink (Q/E)
        if keyboard_input.just_pressed(KeyCode::KeyQ) {
            let left = -transform.local_x();
            transform.translation += left * player.blink_distance;
        }
        if keyboard_input.just_pressed(KeyCode::KeyE) {
            let right = transform.local_x();
            transform.translation += right * player.blink_distance;
        }
    }
}
