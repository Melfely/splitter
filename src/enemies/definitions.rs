use bevy::prelude::*;

/// Unified health and resistance stats. Attach to any entity capable of taking damage.
#[derive(Component, Clone, Copy, Debug)]
pub struct Durability {
    pub hp: f32,
    pub max_hp: f32,
    pub hardness: f32,
    pub bulk: f32,
}

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
