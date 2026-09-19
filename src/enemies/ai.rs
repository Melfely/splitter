use super::definitions::{EnemyAI, EnemyBehavior, EnemyLimb, EnemyMainBody, EnemyStats, LimbKind};
use crate::physics::definitions::{CELL_SIZE, SpatialGrid}; // Uses existing spatial grid infrastructure[cite: 1]
use crate::player::Player;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn update_enemy_ai(
    time: Res<Time>,
    grid: Res<SpatialGrid>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<
        (
            Entity,
            &mut Transform,
            &EnemyAI,
            &EnemyStats,
            Option<&Children>,
        ),
        (With<EnemyMainBody>, Without<Player>),
    >,
    limb_query: Query<&EnemyLimb>,
    mut pos_cache: Local<HashMap<Entity, Vec2>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();

    // Cache enemy body positions
    pos_cache.clear();
    for (entity, transform, ..) in enemy_query.iter() {
        pos_cache.insert(entity, transform.translation.truncate());
    }

    const REPEL_RADIUS: f32 = 48.0;
    const REPEL_RADIUS_SQ: f32 = REPEL_RADIUS * REPEL_RADIUS;
    let time_secs = time.elapsed_secs();

    for (entity, mut transform, ai, stats, children_opt) in enemy_query.iter_mut() {
        let enemy_pos = transform.translation.truncate();
        let to_player = player_pos - enemy_pos;
        let dist_to_player = to_player.length();

        if dist_to_player < 0.001 {
            continue;
        }
        let chase_direction = to_player / dist_to_player;

        let tangent_left = Vec2::new(-chase_direction.y, chase_direction.x);

        // --- Spatial Sensing ---
        let cell_x = (enemy_pos.x / CELL_SIZE).floor() as i32;
        let cell_y = (enemy_pos.y / CELL_SIZE).floor() as i32;

        let mut repel_force = Vec2::ZERO;
        let mut blockage_score: f32 = 0.0;
        let mut total_neighbors: usize = 0;
        let mut flank_dir = Vec2::ZERO;

        for dx in -1..=1 {
            for dy in -1..=1 {
                let cell_key = (cell_x + dx, cell_y + dy);
                if let Some(cell_entities) = grid.cells.get(&cell_key) {
                    for &neighbor_entity in cell_entities {
                        if neighbor_entity == entity {
                            continue;
                        }

                        if let Some(&neighbor_pos) = pos_cache.get(&neighbor_entity) {
                            let diff = enemy_pos - neighbor_pos;
                            let dist_sq = diff.length_squared();

                            if dist_sq < REPEL_RADIUS_SQ && dist_sq > 0.0001 {
                                total_neighbors += 1;
                                let dist = dist_sq.sqrt();
                                let to_neighbor = -diff / dist;
                                let closeness = 1.0 - dist / REPEL_RADIUS;

                                let dist_neighbor_to_player = (player_pos - neighbor_pos).length();
                                let ahead_dot = to_neighbor.dot(chase_direction);

                                // Detect blocking neighbors ahead in the path
                                if dist_neighbor_to_player < dist_to_player && ahead_dot > 0.1 {
                                    blockage_score += closeness * ahead_dot;

                                    let cross_z = chase_direction.x * to_neighbor.y
                                        - chase_direction.y * to_neighbor.x;
                                    let steer_side = if cross_z > 0.0 { -1.0 } else { 1.0 };
                                    flank_dir += tangent_left * (steer_side * closeness);
                                }

                                repel_force += (diff / dist) * closeness;
                            }
                        }
                    }
                }
            }
        }

        // --- Pressure & De-Jamming Flow ---
        // 1. Trailing entities drop inward push to ZERO to stop compressing the core
        let is_blocked = blockage_score > 0.15;
        let forward_weight = if is_blocked { 0.0 } else { 1.0 };

        // 2. Dynamic noise breaks crystalline Close-Packing lattice symmetry
        let entity_id = entity.index().index() as f32;
        let noise_angle = (entity_id * 12.9898 + time_secs * 1.5).sin() * 0.35;
        let orbit_sign = if (entity.index().index() % 2) == 0 {
            1.0
        } else {
            -1.0
        };
        let orbit_vector = tangent_left * orbit_sign;

        let cos_n = noise_angle.cos();
        let sin_n = noise_angle.sin();
        let noisy_orbit = Vec2::new(
            orbit_vector.x * cos_n - orbit_vector.y * sin_n,
            orbit_vector.x * sin_n + orbit_vector.y * cos_n,
        );

        let flank_weight = (blockage_score * 1.5).min(2.5);
        let density_factor = (total_neighbors as f32 / 5.0).clamp(0.0, 2.0);
        let repel_weight = 0.8 + (2.0 * density_factor);

        let blended_vector = match ai.behavior {
            EnemyBehavior::ChasePlayer => {
                let forward_vec = chase_direction * forward_weight;
                let steer_vec = if flank_dir != Vec2::ZERO {
                    flank_dir.normalize() * flank_weight
                } else {
                    noisy_orbit * flank_weight
                };

                forward_vec + steer_vec + (repel_force * repel_weight)
            }
            EnemyBehavior::FleePlayer => -chase_direction + (repel_force * repel_weight),
        };

        let move_direction = blended_vector.normalize_or_zero();
        if move_direction == Vec2::ZERO {
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

        transform.translation += move_direction.extend(0.0) * current_speed;
    }
}
