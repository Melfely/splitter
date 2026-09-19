use super::definitions::{
    Durability, EnemyBehavior, EnemyStats, EnemyTemplate, LimbKind, LimbTemplate,
};

use super::spawner::spawn_enemy;
use bevy::prelude::*;

// Re-use the existing spawner logic and templates

/// Spawns a fast, fragile enemy: Red body with 3 equal-sized green motion limbs arranged evenly around it.
pub fn spawn_fast_red_charger(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    spawn_pos: Vec2,
) -> Entity {
    let red_color = Color::srgb(0.9, 0.1, 0.1);
    let green_color = Color::srgb(0.1, 0.8, 0.2);

    let template = EnemyTemplate {
        durability: Durability {
            hp: 30.0,
            max_hp: 30.0,
            hardness: 5.0,
            bulk: 10.0,
        },
        stats: EnemyStats { base_speed: 100.0 },
        behavior: EnemyBehavior::ChasePlayer,
        body_radius: 18.0,
        color: red_color,
        limbs: vec![
            // Top Limb
            LimbTemplate {
                kind: LimbKind::Motion,
                durability: Durability {
                    hp: 10.0,
                    max_hp: 10.0,
                    hardness: 2.0,
                    bulk: 5.0,
                },
                radius: 10.0,
                local_offset: Vec2::new(0.0, 22.0),
                color: green_color,
                armor: None,
            },
            // Bottom-Left Limb
            LimbTemplate {
                kind: LimbKind::Motion,
                durability: Durability {
                    hp: 10.0,
                    max_hp: 10.0,
                    hardness: 2.0,
                    bulk: 5.0,
                },
                radius: 10.0,
                local_offset: Vec2::new(-19.0, -11.0),
                color: green_color,
                armor: None,
            },
            // Bottom-Right Limb
            LimbTemplate {
                kind: LimbKind::Motion,
                durability: Durability {
                    hp: 10.0,
                    max_hp: 10.0,
                    hardness: 2.0,
                    bulk: 5.0,
                },
                radius: 10.0,
                local_offset: Vec2::new(19.0, -11.0),
                color: green_color,
                armor: None,
            },
        ],
    };

    spawn_enemy(commands, meshes, materials, template, spawn_pos)
}

/// Spawns a slow, high-HP enemy: Yellow body with 8 yellow limbs (middle 4 larger, outer 4 smaller).
pub fn spawn_heavy_yellow_tank(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    spawn_pos: Vec2,
) -> Entity {
    let yellow_body_color = Color::srgb(0.9, 0.8, 0.1);
    let yellow_limb_color = Color::srgb(0.75, 0.65, 0.1);

    let limb_durability_large = Durability {
        hp: 40.0,
        max_hp: 40.0,
        hardness: 15.0,
        bulk: 25.0,
    };
    let limb_durability_small = Durability {
        hp: 20.0,
        max_hp: 20.0,
        hardness: 10.0,
        bulk: 15.0,
    };

    let template = EnemyTemplate {
        durability: Durability {
            hp: 300.0,
            max_hp: 300.0,
            hardness: 30.0,
            bulk: 60.0,
        },
        stats: EnemyStats { base_speed: 75.0 },
        behavior: EnemyBehavior::ChasePlayer,
        body_radius: 32.0,
        color: yellow_body_color,
        limbs: vec![
            // Left Side (Top to Bottom)
            LimbTemplate {
                // Outer Small
                kind: LimbKind::Motion,
                durability: limb_durability_small,
                radius: 8.0,
                local_offset: Vec2::new(-32.0, 30.0),
                color: yellow_limb_color,
                armor: None,
            },
            LimbTemplate {
                // Middle Large
                kind: LimbKind::Motion,
                durability: limb_durability_large,
                radius: 14.0,
                local_offset: Vec2::new(-36.0, 10.0),
                color: yellow_limb_color,
                armor: None,
            },
            LimbTemplate {
                // Middle Large
                kind: LimbKind::Motion,
                durability: limb_durability_large,
                radius: 14.0,
                local_offset: Vec2::new(-36.0, -10.0),
                color: yellow_limb_color,
                armor: None,
            },
            LimbTemplate {
                // Outer Small
                kind: LimbKind::Motion,
                durability: limb_durability_small,
                radius: 8.0,
                local_offset: Vec2::new(-32.0, -30.0),
                color: yellow_limb_color,
                armor: None,
            },
            // Right Side (Top to Bottom)
            LimbTemplate {
                // Outer Small
                kind: LimbKind::Motion,
                durability: limb_durability_small,
                radius: 8.0,
                local_offset: Vec2::new(32.0, 30.0),
                color: yellow_limb_color,
                armor: None,
            },
            LimbTemplate {
                // Middle Large
                kind: LimbKind::Motion,
                durability: limb_durability_large,
                radius: 14.0,
                local_offset: Vec2::new(36.0, 10.0),
                color: yellow_limb_color,
                armor: None,
            },
            LimbTemplate {
                // Middle Large
                kind: LimbKind::Motion,
                durability: limb_durability_large,
                radius: 14.0,
                local_offset: Vec2::new(36.0, -10.0),
                color: yellow_limb_color,
                armor: None,
            },
            LimbTemplate {
                // Outer Small
                kind: LimbKind::Motion,
                durability: limb_durability_small,
                radius: 8.0,
                local_offset: Vec2::new(32.0, -30.0),
                color: yellow_limb_color,
                armor: None,
            },
        ],
    };

    spawn_enemy(commands, meshes, materials, template, spawn_pos)
}
