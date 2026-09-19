use bevy::prelude::*;

use std::collections::HashSet;

use crate::physics::definitions::*;

pub fn check_collisions(
    grid: Res<SpatialGrid>,
    query: Query<(Entity, &GlobalTransform, &Collider)>,
    mut collision_messages: MessageWriter<CollisionMessage>,
) {
    let mut checked = HashSet::new();

    for (entity_a, transform_a, collider_a) in query.iter() {
        let pos_a = transform_a.translation().truncate();
        let cell_x = (pos_a.x / CELL_SIZE).floor() as i32;
        let cell_y = (pos_a.y / CELL_SIZE).floor() as i32;

        for dx in -1..=1 {
            for dy in -1..=1 {
                let neighbor_cell = (cell_x + dx, cell_y + dy);

                if let Some(neighbors) = grid.cells.get(&neighbor_cell) {
                    for &entity_b in neighbors {
                        if entity_a == entity_b {
                            continue;
                        }

                        let pair = if entity_a < entity_b {
                            (entity_a, entity_b)
                        } else {
                            (entity_b, entity_a)
                        };

                        if !checked.insert(pair) {
                            continue;
                        }

                        let Ok((_, transform_b, collider_b)) = query.get(entity_b) else {
                            continue;
                        };

                        if !should_collide(collider_a.layer, collider_b.layer) {
                            continue;
                        }

                        let pos_b = transform_b.translation().truncate();
                        let distance_sq = pos_a.distance_squared(pos_b);
                        let radius_sum = collider_a.radius + collider_b.radius;

                        if distance_sq < radius_sum * radius_sum {
                            collision_messages.write(CollisionMessage { entity_a, entity_b });
                        }
                    }
                }
            }
        }
    }
}

/// Helper function to enforce your game's collision rules
pub fn should_collide(layer_a: CollisionLayer, layer_b: CollisionLayer) -> bool {
    use CollisionLayer::*;
    match (layer_a, layer_b) {
        // Player hull vs Enemy parts
        (Player, EnemyMainBody) | (EnemyMainBody, Player) => true,
        (Player, EnemyLimb) | (EnemyLimb, Player) => true,
        // Weapons collide with all enemy parts[cite: 1]
        (PlayerProjectile, EnemyMainBody) | (EnemyMainBody, PlayerProjectile) => true,
        (PlayerProjectile, EnemyLimb) | (EnemyLimb, PlayerProjectile) => true,

        // Only main bodies collide with each other[cite: 1]
        (EnemyMainBody, EnemyMainBody) => true,

        // Limbs ignore other limbs and bodies
        (EnemyLimb, EnemyLimb) | (EnemyLimb, EnemyMainBody) | (EnemyMainBody, EnemyLimb) => false,

        _ => false,
    }
}

/// Returns the distance along the ray (origin + dir * d) where it first hits a circle collider.
pub fn ray_circle_intersection(origin: Vec2, dir: Vec2, center: Vec2, radius: f32) -> Option<f32> {
    let to_center = center - origin;
    let proj = to_center.dot(dir);

    // Circle is behind the turret barrel
    if proj < 0.0 {
        return None;
    }

    let perp_sq = to_center.length_squared() - proj * proj;
    let radius_sq = radius * radius;

    // Ray misses the circle entirely
    if perp_sq > radius_sq {
        return None;
    }

    // Distance from ray origin to the circle entry point
    let d = proj - (radius_sq - perp_sq).sqrt();
    if d >= 0.0 { Some(d) } else { None }
}
