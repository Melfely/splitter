use crate::physics::definitions::CollisionLayer;
use crate::splitter_core::weapon::{ProjectileState, Weapon, WeaponKind};
use bevy::prelude::*;

/// Rapid-firing, multi-round magazine autocannon.
pub fn create_autocannon(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) -> Weapon {
    Weapon {
        kind: WeaponKind::Projectile {
            mass: 10.0,
            speed: 380.0,
            radius: 5.0,
            layer: CollisionLayer::PlayerProjectile,
            aoe_split: None,
            can_pierce: false,
            aoe_max_range: None,
            current_ammo: 10,
            max_ammo: 10,
            fire_delay: 0.12,
            reload_delay: 1.8,
            state: ProjectileState::Ready,
            mesh: meshes.add(Rectangle::new(5.0, 10.0)),
            material: materials.add(Color::srgb(1.0, 0.6, 0.1)),
        },
    }
}
