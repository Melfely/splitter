use super::definitions::{PlayerPhysics, PlayerShield};
use crate::enemies::definitions::{EnemyMainBody, EnemyStats};
use crate::physics::definitions::{Collider, CollisionLayer, CollisionMessage};
use crate::player::Player;
use crate::splitter_core::projectile::{Durability, Projectile};
use bevy::prelude::*;

pub fn handle_player_collisions(
    time: Res<Time>,
    mut commands: Commands,
    mut collision_messages: MessageReader<CollisionMessage>,
    mut player_root_query: Query<(&mut Transform, &mut PlayerPhysics), With<Player>>,
    mut player_target_query: Query<
        (
            Entity,
            &GlobalTransform,
            &Collider,
            &mut Durability,
            Option<&ChildOf>,
        ),
        (
            Or<(With<Player>, With<PlayerShield>)>,
            Without<EnemyMainBody>,
        ),
    >,
    mut enemy_query: Query<
        (
            &mut Transform,
            &GlobalTransform,
            &Collider,
            &EnemyStats,
            &Durability,
        ),
        (With<EnemyMainBody>, Without<Player>, Without<PlayerShield>),
    >,
    mut projectile_query: Query<(&mut Projectile, &Collider)>,
) {
    let delta_time = time.delta_secs();

    for message in collision_messages.read() {
        let (player_entity, other_entity) = if player_target_query.contains(message.entity_a) {
            (message.entity_a, message.entity_b)
        } else if player_target_query.contains(message.entity_b) {
            (message.entity_b, message.entity_a)
        } else {
            continue;
        };

        let Ok((_, player_global_tf, player_collider, mut target_durability, child_of_opt)) =
            player_target_query.get_mut(player_entity)
        else {
            continue;
        };

        let root_player_entity = child_of_opt.map(|c| c.parent()).unwrap_or(player_entity);

        // --- CASE 1: Physical Enemy Contact ---
        if let Ok((mut enemy_tf, enemy_global_tf, enemy_collider, enemy_stats, enemy_durability)) =
            enemy_query.get_mut(other_entity)
        {
            let player_pos = player_global_tf.translation().truncate();
            let enemy_pos = enemy_global_tf.translation().truncate();

            let delta = player_pos - enemy_pos;
            let distance = delta.length();
            let min_distance = player_collider.radius + enemy_collider.radius;
            let overlap = min_distance - distance;

            if overlap > 0.0 {
                let direction = if distance > 0.001 {
                    delta / distance
                } else {
                    Vec2::Y
                };

                if let Ok((mut player_tf, mut player_physics)) =
                    player_root_query.get_mut(root_player_entity)
                {
                    let player_speed = player_physics.velocity.length();

                    // Battering ram multiplier: Faster tank momentum flings enemies far away
                    let battering_ram_factor = 1.0 + (player_speed / 150.0).min(3.5);

                    // Dominant player authority: Player barely budges (2%), Enemy receives full pushback
                    let player_push_weight = 0.02;
                    let enemy_push_weight = 0.98 * battering_ram_factor;

                    // 1. Positional separation
                    player_tf.translation +=
                        (direction * (overlap * player_push_weight)).extend(0.0);
                    enemy_tf.translation -= (direction * (overlap * enemy_push_weight)).extend(0.0);

                    // 2. Subtle recoil dampening on player velocity & angular spin
                    let minor_recoil = direction * (overlap / delta_time.max(0.001)) * 0.01;
                    player_physics.velocity += minor_recoil;

                    let cross_torque = (enemy_pos - player_pos).perp_dot(direction);
                    player_physics.angular_velocity += cross_torque * 0.005;
                }

                // Continuous contact damage
                let effective_speed = (enemy_stats.base_speed / 100.0).max(0.1);
                let base_damage = effective_speed * enemy_durability.bulk;
                let mass_bulk_ratio = enemy_durability.bulk / target_durability.bulk.max(0.001);
                let speed_hardness_ratio = effective_speed / target_durability.hardness.max(0.001);

                let contact_damage =
                    base_damage * mass_bulk_ratio * speed_hardness_ratio * delta_time;
                target_durability.hp = (target_durability.hp - contact_damage).max(0.0);
            }
        }

        // --- CASE 2: Enemy Projectile Hit ---
        if let Ok((mut projectile, proj_collider)) = projectile_query.get_mut(other_entity) {
            if proj_collider.layer != CollisionLayer::EnemyProjectile {
                continue;
            }

            if !projectile.hit_entities.insert(player_entity) {
                continue;
            }

            // Minimal impact force on tank
            if let Ok((_, mut player_physics)) = player_root_query.get_mut(root_player_entity) {
                let proj_direction = projectile.speed.signum() * Vec2::Y;
                player_physics.velocity += proj_direction * (projectile.mass * 0.5);
            }

            let effective_speed = projectile.speed / 100.0;
            let base_damage = effective_speed * projectile.mass;
            let mass_bulk_ratio = projectile.mass / target_durability.bulk.max(0.001);
            let speed_hardness_ratio = effective_speed / target_durability.hardness.max(0.001);

            let calculated_damage = base_damage * mass_bulk_ratio * speed_hardness_ratio;
            target_durability.hp = (target_durability.hp - calculated_damage).max(0.0);

            if !projectile.can_pierce {
                commands.entity(other_entity).despawn();
            } else {
                projectile.speed -= projectile.speed * 0.25;
                if projectile.speed <= (projectile.initial_speed * 0.10) {
                    commands.entity(other_entity).despawn();
                }
            }
        }
    }
}
