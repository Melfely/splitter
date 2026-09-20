use crate::physics::definitions::CollisionLayer;
use crate::splitter_core::weapon::{ProjectileState, Weapon, WeaponKind};
use bevy::prelude::*;

/// Single-shot, high mass, armor-piercing heavy cannon.
pub fn create_heavy_cannon(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) -> Weapon {
    Weapon {
        kind: WeaponKind::Projectile {
            mass: 35.0,
            speed: 350.0,
            radius: 8.0,
            layer: CollisionLayer::PlayerProjectile,
            aoe_split: None,
            can_pierce: true,
            aoe_max_range: None,
            current_ammo: 1,
            max_ammo: 1,
            fire_delay: 0.0,
            reload_delay: 1.25,
            state: ProjectileState::default(),
            mesh: meshes.add(Rectangle::new(8.0, 16.0)),
            material: materials.add(Color::srgb(0.9, 0.8, 0.2)),
            pellets_per_shot: 1,
            spread_angle: 0.0,
        },
    }
}
