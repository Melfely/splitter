use bevy::prelude::*;
use std::collections::HashMap;

pub const CELL_SIZE: f32 = 64.0;

#[derive(Default, Resource)]
pub struct SpatialGrid {
    pub cells: HashMap<(i32, i32), Vec<Entity>>,
}

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
    pub layer: CollisionLayer,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CollisionLayer {
    Player,
    EnemyMainBody,
    EnemyLimb,
    PlayerProjectile,
    EnemyProjectile,
}

#[derive(Message)]
pub struct CollisionMessage {
    pub entity_a: Entity,
    pub entity_b: Entity,
}
