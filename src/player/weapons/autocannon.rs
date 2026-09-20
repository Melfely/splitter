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
            mass: 2.0,
            speed: 380.0,
            radius: 3.0,
            layer: CollisionLayer::PlayerProjectile,
            aoe_split: None,
            can_pierce: false,
            aoe_max_range: None,
            current_ammo: 100,
            max_ammo: 100,
            fire_delay: 0.1,
            reload_delay: 1.8,
            state: ProjectileState::default(),
            mesh: meshes.add(Rectangle::new(3.0, 6.0)),
            material: materials.add(Color::srgb(1.0, 0.6, 0.1)),
            pellets_per_shot: 10,
            spread_angle: 10.0,
        },
    }
}
