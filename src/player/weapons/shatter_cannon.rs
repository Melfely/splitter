use crate::physics::definitions::CollisionLayer;
use crate::splitter_core::projectile::AoeSplitConfig;
use crate::splitter_core::weapon::{ProjectileState, Weapon, WeaponKind};
use bevy::prelude::*;

/// AoE explosive cluster cannon.
pub fn create_shatter_cannon(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) -> Weapon {
    Weapon {
        kind: WeaponKind::Projectile {
            mass: 22.0,
            speed: 260.0,
            radius: 7.0,
            layer: CollisionLayer::PlayerProjectile,
            can_pierce: false,
            aoe_max_range: Some(900.0),
            aoe_split: Some(AoeSplitConfig {
                child_count: 12,         // Spawns 12 shrapnel fragments in a full circle
                child_speed: 400.0,      // High-velocity cluster burst
                child_mass: 4.0,         // Light shrapnel pieces
                child_radius: 3.0,       // Small fragment collision radius
                child_can_pierce: false, // Shrapnel stops on first impact
            }),
            current_ammo: 4,
            max_ammo: 4,
            fire_delay: 0.35,
            reload_delay: 2.0,
            state: ProjectileState::default(),
            mesh: meshes.add(Circle::new(7.0)),
            material: materials.add(Color::srgb(0.95, 0.3, 0.1)),
            pellets_per_shot: 1,
            spread_angle: 0.0,
        },
    }
}
