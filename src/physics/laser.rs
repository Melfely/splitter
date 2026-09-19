use bevy::math::Vec2;
use bevy::prelude::*;

use crate::physics::definitions::*;
use crate::splitter_core::weapon::{Weapon, WeaponKind, WeaponTrigger};

// Assuming CELL_SIZE, SpatialGrid, Collider, CollisionMessage are imported

pub fn check_laser_collisions(
    grid: Res<SpatialGrid>,
    query: Query<(Entity, &GlobalTransform, &Collider)>,
    // 1. Add Entity to the laser query
    mut laser_query: Query<(Entity, &GlobalTransform, &Weapon, &WeaponTrigger)>,
    mut collision_messages: MessageWriter<CollisionMessage>,
    time: Res<Time>,
) {
    // 2. Destructure weapon_entity from the iterator
    for (weapon_entity, transform, weapon, trigger) in laser_query.iter_mut() {
        if !trigger.is_firing {
            continue;
        }

        let WeaponKind::Laser {
            pattern,
            is_recharging,
            ..
        } = &weapon.kind
        else {
            continue;
        };
        if *is_recharging {
            continue;
        }

        let origin = transform.translation().truncate();
        let direction = transform.up().truncate();
        let points = pattern.generate_points(origin, direction, time.elapsed_secs());
        let mut hit_entities = std::collections::HashSet::new();

        for window in points.windows(2) {
            let p1 = window[0];
            let p2 = window[1];

            let min_x = (p1.x.min(p2.x) / CELL_SIZE).floor() as i32;
            let max_x = (p1.x.max(p2.x) / CELL_SIZE).floor() as i32;
            let min_y = (p1.y.min(p2.y) / CELL_SIZE).floor() as i32;
            let max_y = (p1.y.max(p2.y) / CELL_SIZE).floor() as i32;

            for cx in min_x..=max_x {
                for cy in min_y..=max_y {
                    if let Some(neighbors) = grid.cells.get(&(cx, cy)) {
                        for &entity in neighbors {
                            if !hit_entities.insert(entity) {
                                continue;
                            }

                            let Ok((_, target_transform, target_collider)) = query.get(entity)
                            else {
                                continue;
                            };

                            let target_pos = target_transform.translation().truncate();
                            let dist_sq = distance_squared_to_segment(target_pos, p1, p2);

                            let radius = target_collider.radius;
                            if dist_sq <= radius * radius {
                                collision_messages.write(CollisionMessage {
                                    entity_a: weapon_entity, // 3. Use the extracted entity directly
                                    entity_b: entity,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}
/// Returns the shortest squared distance between a point and a line segment (A -> B).
fn distance_squared_to_segment(point: Vec2, a: Vec2, b: Vec2) -> f32 {
    let line_vec = b - a;
    let point_vec = point - a;
    let line_len_sq = line_vec.length_squared();

    if line_len_sq == 0.0 {
        return point_vec.length_squared(); // A and B are the same point
    }

    // Project point onto the line to find the closest parameter (t)
    let t = (point_vec.dot(line_vec) / line_len_sq).clamp(0.0, 1.0);
    let projection = a + line_vec * t;

    (point - projection).length_squared()
}
