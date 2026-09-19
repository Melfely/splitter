use super::definitions::{EnemyAI, EnemyBehavior, EnemyLimb, EnemyStats, LimbKind};
use crate::player::Player;
use bevy::prelude::*;

pub fn update_enemy_ai(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<
        (&mut Transform, &EnemyAI, &EnemyStats, Option<&Children>),
        Without<Player>,
    >,
    limb_query: Query<&EnemyLimb>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();

    for (mut transform, ai, stats, children_opt) in enemy_query.iter_mut() {
        let enemy_pos = transform.translation.truncate();
        let direction = (player_pos - enemy_pos).normalize_or_zero();

        if direction == Vec2::ZERO {
            continue;
        }

        let mut motion_limb_count = 0;
        if let Some(children) = children_opt {
            for child in children.iter() {
                if let Ok(limb) = limb_query.get(child) {
                    if limb.kind == LimbKind::Motion {
                        motion_limb_count += 1;
                    }
                }
            }
        }

        let speed_multiplier = (motion_limb_count as f32 / 2.0).clamp(0.2, 1.0);
        let current_speed = stats.base_speed * speed_multiplier * time.delta_secs();

        match ai.behavior {
            EnemyBehavior::ChasePlayer => {
                transform.translation += direction.extend(0.0) * current_speed;
            }
            EnemyBehavior::FleePlayer => {
                transform.translation -= direction.extend(0.0) * current_speed;
            }
        }
    }
}
