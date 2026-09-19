use super::definitions::{
    Durability, EnemyAI, EnemyArmor, EnemyBehavior, EnemyLimb, EnemyMainBody, EnemyStats,
    EnemyVisuals, LimbKind,
};
use crate::physics::definitions::{Collider, CollisionLayer};
use crate::splitter_core::turret::{Turret, TurretTarget};
use bevy::prelude::*;

pub struct EnemyTemplate {
    pub durability: Durability,
    pub stats: EnemyStats,
    pub behavior: EnemyBehavior,
    pub body_radius: f32,
    pub color: Color,
    pub limbs: Vec<LimbTemplate>,
}

pub struct LimbTemplate {
    pub kind: LimbKind,
    pub durability: Durability,
    pub radius: f32,
    pub local_offset: Vec2,
    pub color: Color,
    pub armor: Option<ArmorTemplate>,
}

pub struct ArmorTemplate {
    pub durability: Durability,
    pub thickness: f32,
    pub color: Color,
}

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
        ))
        .with_children(|parent| {
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

                if let Some(armor_temp) = limb_temp.armor {
                    limb_entity.with_children(|limb_parent| {
                        limb_parent.spawn((
                            EnemyArmor,
                            EnemyVisuals,
                            armor_temp.durability,
                            Mesh2d(meshes.add(Annulus::new(
                                limb_temp.radius,
                                limb_temp.radius + armor_temp.thickness,
                            ))),
                            MeshMaterial2d(materials.add(armor_temp.color)),
                            Transform::from_translation(Vec3::Z * 0.1),
                        ));
                    });
                }
            }
        })
        .id()
}
