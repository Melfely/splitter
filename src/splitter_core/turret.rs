use bevy::prelude::*;

#[derive(Component)]
pub struct Turret {
    pub turn_speed: f32,
}

#[derive(Component, Default)]
pub struct TurretTarget {
    pub world_pos: Option<Vec2>,
}

pub fn update_turret_aiming(
    time: Res<Time>,
    mut turrets: Query<(&mut Transform, &GlobalTransform, &Turret, &TurretTarget)>,
) {
    for (mut transform, global_transform, turret, target) in turrets.iter_mut() {
        if let Some(target_pos) = target.world_pos {
            let current_world_pos = global_transform.translation().truncate();
            let direction = (target_pos - current_world_pos).normalize_or_zero();

            if direction != Vec2::ZERO {
                // 1. Calculate the desired rotation in absolute WORLD space
                let target_angle = direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2;
                let target_world_rotation = Quat::from_rotation_z(target_angle);

                // 2. Mathematically extract the parent's world rotation using the turret's current transforms
                let current_global_rotation = global_transform.compute_transform().rotation;
                let parent_global_rotation = current_global_rotation * transform.rotation.inverse();

                // 3. Convert the target world rotation into the correct local rotation
                let target_local_rotation =
                    parent_global_rotation.inverse() * target_world_rotation;

                // 4. Interpolate the LOCAL transform towards the new LOCAL target
                transform.rotation = transform
                    .rotation
                    .slerp(target_local_rotation, turret.turn_speed * time.delta_secs())
                    .normalize(); // Prevents quaternion length drift
            }
        }
    }
}
