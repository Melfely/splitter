use super::definitions::{
    EnemyAI, EnemyArmor, EnemyLimb, EnemyMainBody, EnemyTemplate, EnemyVisuals, LimbKind,
};
use crate::GameState;
use crate::physics::definitions::{Collider, CollisionLayer};
use crate::splitter_core::turret::{Turret, TurretTarget};
use bevy::prelude::*;

pub fn spawn_enemy(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    template: EnemyTemplate,
    spawn_pos: Vec2,
) -> Entity {
    commands
        .spawn((
            EnemyMainBody,
            EnemyVisuals,
            template.durability,
            template.stats,
            EnemyAI {
                behavior: template.behavior,
            },
            Mesh2d(meshes.add(Circle::new(template.body_radius))),
            MeshMaterial2d(materials.add(template.color)),
            Transform::from_translation(spawn_pos.extend(0.0)),
            Collider {
                radius: template.body_radius,
                layer: CollisionLayer::EnemyMainBody,
            },
            DespawnOnEnter(GameState::MainMenu),
        ))
        .with_children(|parent| {
            // --- Body Armor Plates (No Colliders) ---
            if let Some(body_armor) = template.body_armor {
                for armor_temp in body_armor {
                    parent.spawn((
                        EnemyArmor,
                        EnemyVisuals,
                        armor_temp.durability,
                        Mesh2d(meshes.add(Circle::new(armor_temp.radius))),
                        MeshMaterial2d(materials.add(armor_temp.color)),
                        Transform::from_translation(armor_temp.local_offset.extend(0.2)),
                    ));
                }
            }

            // --- Limbs & Limb Armor Plates ---
            for limb_temp in template.limbs {
                let mut limb_entity = parent.spawn((
                    EnemyLimb {
                        kind: limb_temp.kind,
                    },
                    EnemyVisuals,
                    limb_temp.durability,
                    Mesh2d(meshes.add(Circle::new(limb_temp.radius))),
                    MeshMaterial2d(materials.add(limb_temp.color)),
                    Transform::from_translation(limb_temp.local_offset.extend(0.1)),
                    Collider {
                        radius: limb_temp.radius,
                        layer: CollisionLayer::EnemyLimb,
                    },
                ));

                if limb_temp.kind == LimbKind::RangedAttack {
                    limb_entity.insert((Turret { turn_speed: 3.0 }, TurretTarget::default()));
                }

                // Limb Armor (No Colliders)
                if let Some(limb_armor) = limb_temp.armor {
                    limb_entity.with_children(|limb_parent| {
                        for armor_temp in limb_armor {
                            limb_parent.spawn((
                                EnemyArmor,
                                EnemyVisuals,
                                armor_temp.durability,
                                Mesh2d(meshes.add(Circle::new(armor_temp.radius))),
                                MeshMaterial2d(materials.add(armor_temp.color)),
                                Transform::from_translation(armor_temp.local_offset.extend(0.1)),
                            ));
                        }
                    });
                }
            }
        })
        .id()
}
