use super::definitions::{
    ArmorTemplate, EnemyBehavior, EnemyStats, EnemyTemplate, LimbKind, LimbTemplate,
};

use crate::splitter_core::projectile::Durability;

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
        body_armor: None,
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
        body_armor: None,
        durability: Durability {
            hp: 300.0,
            max_hp: 300.0,
            hardness: 1.0,
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

/// Spawns a special armored fast enemy:
/// Red body with 1 body armor plate and 3 green motion limbs (1 armor plate per limb).
pub fn spawn_fast_red_charger_special(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    spawn_pos: Vec2,
) -> Entity {
    let red_color = Color::srgb(0.9, 0.1, 0.1);
    let green_color = Color::srgb(0.1, 0.8, 0.2);
    let armor_color = Color::srgb(0.35, 0.35, 0.4);

    let limb_armor = ArmorTemplate {
        durability: Durability {
            hp: 15.0,
            max_hp: 15.0,
            hardness: 8.0,
            bulk: 5.0,
        },
        radius: 6.0,
        local_offset: Vec2::new(0.0, 8.0),
        color: armor_color,
    };

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
        body_armor: Some(vec![ArmorTemplate {
            durability: Durability {
                hp: 25.0,
                max_hp: 25.0,
                hardness: 12.0,
                bulk: 10.0,
            },
            radius: 12.0,
            local_offset: Vec2::new(0.0, 12.0),
            color: armor_color,
        }]),
        limbs: vec![
            // Top Limb (1 Armor)
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
                armor: Some(vec![limb_armor.clone()]),
            },
            // Bottom-Left Limb (1 Armor)
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
                armor: Some(vec![limb_armor.clone()]),
            },
            // Bottom-Right Limb (1 Armor)
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
                armor: Some(vec![limb_armor.clone()]),
            },
        ],
    };

    spawn_enemy(commands, meshes, materials, template, spawn_pos)
}

/// Spawns a heavy armored tank variant:
/// Yellow body with 3 body armor plates and 8 yellow limbs (3 armor plates per limb).
pub fn spawn_heavy_yellow_tank_special(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    spawn_pos: Vec2,
) -> Entity {
    let yellow_body_color = Color::srgb(0.9, 0.8, 0.1);
    let yellow_limb_color = Color::srgb(0.75, 0.65, 0.1);
    let heavy_armor_color = Color::srgb(0.25, 0.25, 0.3);

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

    // Helper closure to generate 3 armor plates per limb spaced around its outer edge
    let make_triple_limb_armor = |armor_hp: f32, armor_hardness: f32, plate_radius: f32| {
        vec![
            ArmorTemplate {
                durability: Durability {
                    hp: armor_hp,
                    max_hp: armor_hp,
                    hardness: armor_hardness,
                    bulk: 8.0,
                },
                radius: plate_radius,
                local_offset: Vec2::new(-plate_radius * 0.9, plate_radius * 0.5),
                color: heavy_armor_color,
            },
            ArmorTemplate {
                durability: Durability {
                    hp: armor_hp,
                    max_hp: armor_hp,
                    hardness: armor_hardness,
                    bulk: 8.0,
                },
                radius: plate_radius,
                local_offset: Vec2::new(0.0, plate_radius * 1.1),
                color: heavy_armor_color,
            },
            ArmorTemplate {
                durability: Durability {
                    hp: armor_hp,
                    max_hp: armor_hp,
                    hardness: armor_hardness,
                    bulk: 8.0,
                },
                radius: plate_radius,
                local_offset: Vec2::new(plate_radius * 0.9, plate_radius * 0.5),
                color: heavy_armor_color,
            },
        ]
    };

    let small_limb_armor = make_triple_limb_armor(15.0, 15.0, 4.5);
    let large_limb_armor = make_triple_limb_armor(30.0, 22.0, 7.0);

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
        body_armor: Some(vec![
            ArmorTemplate {
                durability: Durability {
                    hp: 60.0,
                    max_hp: 60.0,
                    hardness: 25.0,
                    bulk: 20.0,
                },
                radius: 14.0,
                local_offset: Vec2::new(-16.0, 20.0),
                color: heavy_armor_color,
            },
            ArmorTemplate {
                durability: Durability {
                    hp: 80.0,
                    max_hp: 80.0,
                    hardness: 30.0,
                    bulk: 25.0,
                },
                radius: 18.0,
                local_offset: Vec2::new(0.0, 26.0),
                color: heavy_armor_color,
            },
            ArmorTemplate {
                durability: Durability {
                    hp: 60.0,
                    max_hp: 60.0,
                    hardness: 25.0,
                    bulk: 20.0,
                },
                radius: 14.0,
                local_offset: Vec2::new(16.0, 20.0),
                color: heavy_armor_color,
            },
        ]),
        limbs: vec![
            // Left Side (Top to Bottom)
            LimbTemplate {
                kind: LimbKind::Motion,
                durability: limb_durability_small,
                radius: 8.0,
                local_offset: Vec2::new(-32.0, 30.0),
                color: yellow_limb_color,
                armor: Some(small_limb_armor.clone()),
            },
            LimbTemplate {
                kind: LimbKind::Motion,
                durability: limb_durability_large,
                radius: 14.0,
                local_offset: Vec2::new(-36.0, 10.0),
                color: yellow_limb_color,
                armor: Some(large_limb_armor.clone()),
            },
            LimbTemplate {
                kind: LimbKind::Motion,
                durability: limb_durability_large,
                radius: 14.0,
                local_offset: Vec2::new(-36.0, -10.0),
                color: yellow_limb_color,
                armor: Some(large_limb_armor.clone()),
            },
            LimbTemplate {
                kind: LimbKind::Motion,
                durability: limb_durability_small,
                radius: 8.0,
                local_offset: Vec2::new(-32.0, -30.0),
                color: yellow_limb_color,
                armor: Some(small_limb_armor.clone()),
            },
            // Right Side (Top to Bottom)
            LimbTemplate {
                kind: LimbKind::Motion,
                durability: limb_durability_small,
                radius: 8.0,
                local_offset: Vec2::new(32.0, 30.0),
                color: yellow_limb_color,
                armor: Some(small_limb_armor.clone()),
            },
            LimbTemplate {
                kind: LimbKind::Motion,
                durability: limb_durability_large,
                radius: 14.0,
                local_offset: Vec2::new(36.0, 10.0),
                color: yellow_limb_color,
                armor: Some(large_limb_armor.clone()),
            },
            LimbTemplate {
                kind: LimbKind::Motion,
                durability: limb_durability_large,
                radius: 14.0,
                local_offset: Vec2::new(36.0, -10.0),
                color: yellow_limb_color,
                armor: Some(large_limb_armor.clone()),
            },
            LimbTemplate {
                kind: LimbKind::Motion,
                durability: limb_durability_small,
                radius: 8.0,
                local_offset: Vec2::new(32.0, -30.0),
                color: yellow_limb_color,
                armor: Some(small_limb_armor.clone()),
            },
        ],
    };

    spawn_enemy(commands, meshes, materials, template, spawn_pos)
}
