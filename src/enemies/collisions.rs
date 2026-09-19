use super::definitions::EnemyMainBody;
use crate::physics::definitions::{Collider, CollisionMessage};
use bevy::prelude::*;

/// Resolves collisions by hard-projecting overlapping bodies completely out of each other.
pub fn handle_enemy_body_collisions(
    mut collision_messages: MessageReader<CollisionMessage>,
    mut enemy_query: Query<(&mut Transform, &Collider), With<EnemyMainBody>>,
) {
    for message in collision_messages.read() {
        if let Ok([(mut transform_a, collider_a), (mut transform_b, collider_b)]) =
            enemy_query.get_many_mut([message.entity_a, message.entity_b])
        {
            let pos_a = transform_a.translation.truncate();
            let pos_b = transform_b.translation.truncate();

            let delta = pos_a - pos_b;
            let distance = delta.length();
            let min_distance = collider_a.radius + collider_b.radius;

            let overlap = min_distance - distance;
            if overlap > 0.0 {
                // Determine repulsion vector (fallback to random split if perfectly centered)
                let direction = if distance > 0.001 {
                    delta / distance
                } else {
                    Vec2::new(1.0, 0.0)
                };

                // Full geometric separation: push both entities out by half the overlap immediately
                let separation = direction * (overlap * 0.5);
                transform_a.translation += separation.extend(0.0);
                transform_b.translation -= separation.extend(0.0);
            }
        }
    }
}
