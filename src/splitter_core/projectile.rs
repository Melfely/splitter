use bevy::prelude::*;
use rand::RngExt;
use std::collections::HashSet;

use crate::GameState;
use crate::enemies::definitions::{EnemyAI, EnemyArmor, EnemyLimb, EnemyMainBody, EnemyStats};
use crate::physics::collisions::should_collide;
use crate::physics::definitions::{Collider, CollisionLayer, CollisionMessage};
use crate::splitter_core::Lifetime;
use crate::splitter_core::PLAYER_ATTACK_DISTANCE;
use crate::splitter_core::hit_particle::spawn_impact_sparks;

/// Configuration defining how a projectile splits upon detonation.
#[derive(Clone, Debug)]
pub struct AoeSplitConfig {
    pub child_count: u32,
    pub child_speed: f32,
    pub child_mass: f32,
    pub child_radius: f32,
    pub child_can_pierce: bool,
}

/// Unified health and resistance stats. Attach to any entity capable of taking damage.
#[derive(Component, Clone, Copy, Debug)]
pub struct Durability {
    pub hp: f32,
    pub max_hp: f32,
    pub hardness: f32,
    pub bulk: f32,
}

#[derive(Component)]
pub struct Projectile {
    pub mass: f32,
    pub speed: f32,
    pub initial_speed: f32, // Tracks original max speed for the 10% deletion threshold
    pub hit_entities: HashSet<Entity>,
    pub can_pierce: bool,
    pub aoe_max_range: Option<f32>,
    pub distance_traveled: f32,
    pub aoe_split: Option<AoeSplitConfig>,
}

/// Helper function to detonate an explosive projectile, spawning radial child projectiles with randomized spread.
pub fn detonate_aoe_split(
    commands: &mut Commands,
    epicenter: Vec3,
    layer: CollisionLayer,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
    config: &AoeSplitConfig,
) {
    if config.child_count == 0 {
        return;
    }

    let mut rng = rand::rng();

    // Random base offset so explosions aren't aligned in identical cardinal directions
    let base_angle_offset = rng.random_range(0.0..std::f32::consts::TAU);
    let angle_step = std::f32::consts::TAU / config.child_count as f32;

    for i in 0..config.child_count {
        // 1. Angular Jitter: Slight offset (+/- 20% of step size)
        let angle_jitter = rng.random_range(-angle_step * 0.2..angle_step * 0.2);
        let angle = base_angle_offset + (i as f32 * angle_step) + angle_jitter;
        let rotation = Quat::from_rotation_z(angle);

        // 2. Speed Variance: Speed varies by +/- 15% for organic shrapnel spread
        let speed_multiplier = rng.random_range(0.85..=1.15);
        let actual_speed = config.child_speed * speed_multiplier;

        let lifetime = calculate_lifetime_from_speed(actual_speed);

        commands.spawn((
            Projectile {
                mass: config.child_mass,
                speed: actual_speed,
                initial_speed: actual_speed,
                hit_entities: HashSet::new(),
                can_pierce: config.child_can_pierce,
                aoe_max_range: None,
                distance_traveled: 0.0,
                aoe_split: None, // Fragments do not chain-split by default
            },
            Collider {
                radius: config.child_radius,
                layer,
            },
            lifetime,
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
            Transform::from_xyz(epicenter.x, epicenter.y, 0.0).with_rotation(rotation),
            GlobalTransform::default(),
            DespawnOnEnter(GameState::MainMenu),
        ));
    }
}

pub fn update_projectile_movement(
    time: Res<Time>,
    mut commands: Commands,
    mut projectiles: Query<(
        Entity,
        &mut Transform,
        &mut Projectile,
        &Collider,
        &GlobalTransform,
        &Mesh2d,
        &MeshMaterial2d<ColorMaterial>, // <-- Add <ColorMaterial>
    )>,
) {
    for (entity, mut transform, mut projectile, collider, gt, mesh, mat) in projectiles.iter_mut() {
        let step = transform.up() * projectile.speed * time.delta_secs();

        transform.translation += step;
        projectile.distance_traveled += step.length();

        if let Some(max_range) = projectile.aoe_max_range {
            if projectile.distance_traveled >= max_range {
                if let Some(ref split_config) = projectile.aoe_split {
                    detonate_aoe_split(
                        &mut commands,
                        gt.translation(),
                        collider.layer,
                        mesh.0.clone(),
                        mat.0.clone(),
                        split_config,
                    );
                }
                commands.entity(entity).try_despawn();
            }
        }
    }
}

/// Calculates lifetime based on projectile speed to guarantee off-screen travel before despawning.
pub fn calculate_lifetime_from_speed(speed: f32) -> Lifetime {
    let distance = PLAYER_ATTACK_DISTANCE; // 2500.0 units clears the screen
    let seconds = (distance / speed.max(1.0)).clamp(0.5, 10.0);
    Lifetime(Timer::from_seconds(seconds, TimerMode::Once))
}

// Helper function to check if an entity or any parent in its hierarchy was destroyed this frame.
fn is_entity_or_ancestor_destroyed(
    mut entity: Entity,
    destroyed_set: &HashSet<Entity>,
    child_of_query: &Query<&ChildOf>,
) -> bool {
    if destroyed_set.contains(&entity) {
        return true;
    }
    while let Ok(child_of) = child_of_query.get(entity) {
        let parent = child_of.parent();
        if destroyed_set.contains(&parent) {
            return true;
        }
        entity = parent;
    }
    false
}

pub fn handle_projectile_collisions(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut collision_messages: MessageReader<CollisionMessage>,
    mut projectile_query: Query<(
        &mut Projectile,
        &Collider,
        &GlobalTransform,
        &Mesh2d,
        &MeshMaterial2d<ColorMaterial>,
    )>,
    mut target_query: Query<
        (
            &mut Durability,
            &Collider,
            &GlobalTransform,
            Option<&EnemyLimb>,
            Option<&ChildOf>,
        ),
        Without<EnemyArmor>,
    >,
    children_query: Query<&Children>,
    mut armor_query: Query<(Entity, &mut Durability), With<EnemyArmor>>,
    child_of_query: Query<&ChildOf>,
) {
    let mut destroyed_entities = HashSet::new();

    for message in collision_messages.read() {
        // 1. Identify projectile and target entities
        let (proj_entity, target_entity) = if projectile_query.contains(message.entity_a) {
            (message.entity_a, message.entity_b)
        } else if projectile_query.contains(message.entity_b) {
            (message.entity_b, message.entity_a)
        } else {
            continue;
        };

        if destroyed_entities.contains(&proj_entity)
            || is_entity_or_ancestor_destroyed(target_entity, &destroyed_entities, &child_of_query)
        {
            continue;
        }

        let Ok((mut projectile, proj_collider, proj_global_transform, proj_mesh, proj_mat)) =
            projectile_query.get_mut(proj_entity)
        else {
            continue;
        };
        let Ok((
            mut target_durability,
            target_collider,
            target_global_transform,
            target_limb,
            target_child_of,
        )) = target_query.get_mut(target_entity)
        else {
            continue;
        };

        // 2. Validate collision layers
        if !should_collide(proj_collider.layer, target_collider.layer) {
            continue;
        }

        // 3. Prevent hitting the same target twice with the same projectile
        if !projectile.hit_entities.insert(target_entity) {
            continue;
        }

        // 4. Visual impact sparks
        let impact_pos = target_global_transform.translation();
        let bullet_dir = proj_global_transform.up().truncate();
        let spark_color = if target_durability.hp <= 0.0 {
            Color::srgb(1.0, 0.3, 0.1)
        } else {
            Color::srgb(1.0, 0.9, 0.3)
        };

        spawn_impact_sparks(
            &mut commands,
            &mut meshes,
            &mut materials,
            impact_pos,
            bullet_dir,
            spark_color,
        );

        // 5. Look up attached armor plate (excluding armor already destroyed this frame)
        let attached_armor_entity = children_query.get(target_entity).ok().and_then(|children| {
            children
                .iter()
                .find(|&child| armor_query.contains(child) && !destroyed_entities.contains(&child))
        });

        // 6. Apply Damage Logic
        if let Some(armor_entity) = attached_armor_entity {
            if let Ok((_, mut armor_durability)) = armor_query.get_mut(armor_entity) {
                let effective_speed = projectile.speed / 100.0;
                let base_damage = effective_speed * projectile.mass;
                let mass_bulk_ratio = projectile.mass / armor_durability.bulk.max(0.001);
                let speed_hardness_ratio = effective_speed / armor_durability.hardness.max(0.001);

                let calculated_damage = base_damage * mass_bulk_ratio * speed_hardness_ratio;
                armor_durability.hp -= calculated_damage;

                if armor_durability.hp <= 0.0 {
                    destroyed_entities.insert(armor_entity);
                    commands.entity(armor_entity).try_despawn();
                }

                // Projectile loses speed on armor impact
                projectile.speed -= projectile.speed * 0.25;
            }
        } else {
            // Damage applied directly to Target (Body or Limb)
            let effective_speed = projectile.speed / 100.0;
            let base_damage = effective_speed * projectile.mass;
            let mass_bulk_ratio = projectile.mass / target_durability.bulk.max(0.001);
            let speed_hardness_ratio = effective_speed / target_durability.hardness.max(0.001);

            let calculated_damage = base_damage * mass_bulk_ratio * speed_hardness_ratio;

            let pre_hit_hp = target_durability.hp;
            target_durability.hp -= calculated_damage;

            let target_destroyed = target_durability.hp <= 0.0;

            if target_destroyed {
                destroyed_entities.insert(target_entity);

                let overkill_factor = (pre_hit_hp / calculated_damage.max(0.001)).clamp(0.0, 1.0);
                let speed_lost = projectile.speed * 0.25 * overkill_factor;
                projectile.speed -= speed_lost;

                if target_limb.is_some() && speed_lost > 1.0 {
                    const SPEED_BOOST_MULTIPLIER: f32 = 4.0;
                    let limb_speed = (speed_lost * SPEED_BOOST_MULTIPLIER).max(400.0);
                    let bullet_rotation = proj_global_transform.compute_transform().rotation;

                    let mut hit_entities = HashSet::new();
                    hit_entities.insert(target_entity);

                    // Add parent body and all sibling entities on the same enemy
                    if let Some(child_of) = target_child_of {
                        let parent_entity = child_of.parent();
                        hit_entities.insert(parent_entity);

                        if let Ok(siblings) = children_query.get(parent_entity) {
                            for sibling in siblings.iter() {
                                hit_entities.insert(sibling);
                            }
                        }
                    }

                    let limb_lifetime = calculate_lifetime_from_speed(limb_speed);

                    commands
                        .entity(target_entity)
                        .remove_parent_in_place()
                        .insert((
                            Projectile {
                                mass: target_durability.bulk.max(1.0),
                                speed: limb_speed,
                                initial_speed: limb_speed,
                                hit_entities,
                                can_pierce: false,
                                aoe_max_range: None,
                                distance_traveled: 0.0,
                                aoe_split: None,
                            },
                            Collider {
                                radius: target_collider.radius,
                                layer: CollisionLayer::PlayerProjectile,
                            },
                            limb_lifetime,
                            Transform::from_translation(target_global_transform.translation())
                                .with_rotation(bullet_rotation),
                        ))
                        .remove::<(
                            Durability,
                            EnemyMainBody,
                            EnemyLimb,
                            EnemyArmor,
                            EnemyAI,
                            EnemyStats,
                        )>();
                } else {
                    commands.entity(target_entity).try_despawn();
                }
            } else {
                projectile.speed -= projectile.speed * 0.25;
            }
        }

        // 7. Non-piercing projectiles ALWAYS despawn after first impact
        if !projectile.can_pierce {
            if let Some(ref split_config) = projectile.aoe_split {
                detonate_aoe_split(
                    &mut commands,
                    proj_global_transform.translation(),
                    proj_collider.layer,
                    proj_mesh.0.clone(),
                    proj_mat.0.clone(),
                    split_config,
                );
            }

            destroyed_entities.insert(proj_entity);
            commands.entity(proj_entity).try_despawn();
            continue;
        }

        // 8. Piercing projectiles despawn if speed drops to <= 10% of initial speed
        if projectile.speed <= (projectile.initial_speed * 0.10) {
            if let Some(ref split_config) = projectile.aoe_split {
                detonate_aoe_split(
                    &mut commands,
                    proj_global_transform.translation(),
                    proj_collider.layer,
                    proj_mesh.0.clone(),
                    proj_mat.0.clone(),
                    split_config,
                );
            }

            destroyed_entities.insert(proj_entity);
            commands.entity(proj_entity).try_despawn();
        }
    }
}
