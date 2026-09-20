use crate::player::definitions::{
    CoaxialWeapon, MainWeapon, MountLocation, MountPoint, MountedWeapon,
};
use crate::splitter_core::weapon::{Weapon, WeaponKind, WeaponTrigger};
use bevy::prelude::*;

/// Hook to attach an externally defined `MountedWeapon` to an entity parent.
pub fn attach_mounted_weapon(
    parent: &mut ChildSpawnerCommands,
    mounted_weapon: MountedWeapon,
) -> Vec<Entity> {
    attach_weapon(
        parent,
        mounted_weapon.weapon,
        mounted_weapon.mount_point,
        mounted_weapon.mesh,
        mounted_weapon.material,
    )
}

/// System function for attaching weapons with slot stat modifications and input marker hooks.
pub fn attach_weapon(
    parent: &mut ChildSpawnerCommands,
    mut weapon: Weapon,
    mount_point: MountPoint,
    barrel_mesh: Handle<Mesh>,
    barrel_material: Handle<ColorMaterial>,
) -> Vec<Entity> {
    let mut spawned_entities = Vec::new();

    match mount_point {
        MountPoint::Turret { offset } => {
            let id = parent
                .spawn((
                    Mesh2d(barrel_mesh),
                    MeshMaterial2d(barrel_material),
                    Transform::from_translation(offset),
                    WeaponTrigger::default(),
                    weapon,
                    MountLocation::Turret,
                    MainWeapon,
                ))
                .id();
            spawned_entities.push(id);
        }
        MountPoint::Coaxial { offset } => {
            // Coaxial bonus: +100% projectile mass[cite: 1]
            if let WeaponKind::Projectile { ref mut mass, .. } = weapon.kind {
                *mass *= 2.0;
            }

            let id = parent
                .spawn((
                    Mesh2d(barrel_mesh),
                    MeshMaterial2d(barrel_material),
                    Transform::from_translation(offset),
                    WeaponTrigger::default(),
                    weapon,
                    MountLocation::Coaxial,
                    CoaxialWeapon,
                ))
                .id();
            spawned_entities.push(id);
        }
        MountPoint::DualTurret { offset, spacing } => {
            // Dual Wield penalty: 50% reduction in projectile mass[cite: 1]
            if let WeaponKind::Projectile { ref mut mass, .. } = weapon.kind {
                *mass *= 0.5;
            }

            let half_spacing = spacing / 2.0;
            let left_pos = offset + Vec3::new(-half_spacing, 0.0, 0.0);
            let right_pos = offset + Vec3::new(half_spacing, 0.0, 0.0);

            let e1 = parent
                .spawn((
                    Mesh2d(barrel_mesh.clone()),
                    MeshMaterial2d(barrel_material.clone()),
                    Transform::from_translation(left_pos),
                    WeaponTrigger::default(),
                    weapon.clone(),
                    MountLocation::Turret,
                    MainWeapon,
                ))
                .id();

            let e2 = parent
                .spawn((
                    Mesh2d(barrel_mesh),
                    MeshMaterial2d(barrel_material),
                    Transform::from_translation(right_pos),
                    WeaponTrigger::default(),
                    weapon,
                    MountLocation::Turret,
                    MainWeapon,
                ))
                .id();

            spawned_entities.push(e1);
            spawned_entities.push(e2);
        }
        MountPoint::DualCoaxial { offset, spacing } => {
            // Coaxial (+100%) and Dual (-50%) cancel out (x2.0 * x0.5 = x1.0 default mass)
            let half_spacing = spacing / 2.0;
            let left_pos = offset + Vec3::new(-half_spacing, 0.0, 0.0);
            let right_pos = offset + Vec3::new(half_spacing, 0.0, 0.0);

            let e1 = parent
                .spawn((
                    Mesh2d(barrel_mesh.clone()),
                    MeshMaterial2d(barrel_material.clone()),
                    Transform::from_translation(left_pos),
                    WeaponTrigger::default(),
                    weapon.clone(),
                    MountLocation::Coaxial,
                    CoaxialWeapon,
                ))
                .id();

            let e2 = parent
                .spawn((
                    Mesh2d(barrel_mesh),
                    MeshMaterial2d(barrel_material),
                    Transform::from_translation(right_pos),
                    WeaponTrigger::default(),
                    weapon,
                    MountLocation::Coaxial,
                    CoaxialWeapon,
                ))
                .id();

            spawned_entities.push(e1);
            spawned_entities.push(e2);
        }
    }

    spawned_entities
}
