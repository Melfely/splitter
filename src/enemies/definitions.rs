use crate::splitter_core::projectile::Durability;
use bevy::prelude::*;

#[derive(Component)]
pub struct EnemyMainBody;

#[derive(Component)]
pub struct EnemyStats {
    pub base_speed: f32,
}

#[derive(Component)]
pub struct EnemyAI {
    pub behavior: EnemyBehavior,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EnemyBehavior {
    ChasePlayer,
    FleePlayer,
}

#[derive(Component)]
pub struct EnemyLimb {
    pub kind: LimbKind,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LimbKind {
    Motion,
    RangedAttack,
}

#[derive(Component)]
pub struct EnemyArmor;

#[derive(Component)]
pub struct EnemyVisuals;

#[derive(Clone)]
pub struct ArmorTemplate {
    pub durability: Durability,
    pub radius: f32,
    pub local_offset: Vec2,
    pub color: Color,
}

pub struct LimbTemplate {
    pub kind: LimbKind,
    pub durability: Durability,
    pub radius: f32,
    pub local_offset: Vec2,
    pub color: Color,
    pub armor: Option<Vec<ArmorTemplate>>,
}

pub struct EnemyTemplate {
    pub durability: Durability,
    pub stats: EnemyStats,
    pub behavior: EnemyBehavior,
    pub body_radius: f32,
    pub color: Color,
    pub body_armor: Option<Vec<ArmorTemplate>>,
    pub limbs: Vec<LimbTemplate>,
}
