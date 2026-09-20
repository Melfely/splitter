use crate::menu::{GameState, PendingRestart};
use crate::player::definitions::PlayerLoadout;
use crate::player::definitions::{MountPoint, MountedWeapon};
use crate::player::weapons::*;
use crate::splitter_core::weapon::{Weapon, WeaponKind};
use bevy::prelude::*;

// =========================================================================
// 1. EXTENSIBLE WEAPON REGISTRY
// =========================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum WeaponId {
    #[default]
    HeavyCannon,
    Autocannon,
    ShatterCannon,
}

impl WeaponId {
    pub const ALL: &'static [WeaponId] = &[
        WeaponId::HeavyCannon,
        WeaponId::Autocannon,
        WeaponId::ShatterCannon,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            WeaponId::HeavyCannon => "Heavy Cannon",
            WeaponId::Autocannon => "Autocannon",
            WeaponId::ShatterCannon => "Shatter Cannon",
        }
    }

    pub fn build(
        &self,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Weapon {
        match self {
            WeaponId::HeavyCannon => create_heavy_cannon(meshes, materials),
            WeaponId::Autocannon => create_autocannon(meshes, materials),
            WeaponId::ShatterCannon => create_shatter_cannon(meshes, materials),
        }
    }

    pub fn barrel_mesh(&self, meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
        match self {
            WeaponId::HeavyCannon => meshes.add(Rectangle::new(12.0, 50.0)),
            WeaponId::Autocannon => meshes.add(Rectangle::new(10.0, 40.0)),
            WeaponId::ShatterCannon => meshes.add(Circle::new(7.0)),
        }
    }

    pub fn next(&self) -> Self {
        let all = Self::ALL;
        let idx = all.iter().position(|w| w == self).unwrap_or(0);
        all[(idx + 1) % all.len()]
    }

    pub fn prev(&self) -> Self {
        let all = Self::ALL;
        let idx = all.iter().position(|w| w == self).unwrap_or(0);
        all[(idx + all.len() - 1) % all.len()]
    }
}

// =========================================================================
// 2. GEAR SELECTION STATE RESOURCE
// =========================================================================

#[derive(Resource, Clone)]
pub struct SelectedGearState {
    pub turret_weapon: WeaponId,
    pub turret_is_dual: bool,
    pub coaxial_weapon: WeaponId,
    pub coaxial_is_dual: bool,
}

impl Default for SelectedGearState {
    fn default() -> Self {
        Self {
            turret_weapon: WeaponId::HeavyCannon,
            turret_is_dual: false,
            coaxial_weapon: WeaponId::Autocannon,
            coaxial_is_dual: false,
        }
    }
}

// =========================================================================
// 3. UI MARKER COMPONENTS
// =========================================================================

#[derive(Component)]
pub struct TurretMountToggleBtn;
#[derive(Component)]
pub struct TurretPrevBtn;
#[derive(Component)]
pub struct TurretNextBtn;

#[derive(Component)]
pub struct CoaxialMountToggleBtn;
#[derive(Component)]
pub struct CoaxialPrevBtn;
#[derive(Component)]
pub struct CoaxialNextBtn;

#[derive(Component)]
pub struct LaunchMissionBtn;
#[derive(Component)]
pub struct HangarBackBtn;

/// Single enum marker for UI text elements to prevent Query borrowing conflicts (B0001)
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum HangarUiLabel {
    TurretName,
    TurretMount,
    TurretStats,
    CoaxialName,
    CoaxialMount,
    CoaxialStats,
}

// =========================================================================
// 4. STAT FORMATTING HELPERS
// =========================================================================

fn format_weapon_stats(
    weapon_id: WeaponId,
    is_coaxial: bool,
    is_dual: bool,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) -> String {
    let weapon = weapon_id.build(meshes, materials);

    match weapon.kind {
        WeaponKind::Projectile {
            mass,
            speed,
            radius,
            can_pierce,
            aoe_max_range,
            aoe_split,
            max_ammo,
            fire_delay,
            reload_delay,
            ..
        } => {
            let (effective_mass, mass_desc) = match (is_coaxial, is_dual) {
                (true, true) => (mass * 1.0, "Standard Mass (Coax +100% & Dual -50% cancel)"),
                (true, false) => (mass * 2.0, "+100% Mass (Coaxial Bonus)"),
                (false, true) => (mass * 0.5, "-50% Mass (Dual Penalty)"),
                (false, false) => (mass * 1.0, "Standard Mass"),
            };

            let barrel_count = if is_dual { "2x Barrels" } else { "1x Barrel" };
            let impact_damage = (speed / 100.0) * effective_mass;
            let fire_rate_str = if fire_delay > 0.0 {
                format!("{:.1} rnd/s ({:.2}s)", 1.0 / fire_delay, fire_delay)
            } else {
                "Instant".to_string()
            };
            let aoe_str = aoe_max_range.map_or("None".to_string(), |r| format!("{:.0} px", r));
            let split_str = match &aoe_split {
                Some(split) => format!(
                    "Active ({} Fragments)\n\
                    Submunition Mass: {} \n\
                    Submunition Speed: {} \n\
                    Submunition Impact Damage: {} ",
                    split.child_count,
                    split.child_mass,
                    split.child_speed,
                    ((split.child_speed / 100.0) * split.child_mass)
                ),
                None => "None".to_string(),
            };

            format!(
                "Barrels: {}\n\
                Base Mass: {:.1}\n\
                Net Mass: {:.1} ({})\n\
                Impact Damage: {:.1}\n\
                Proj Speed: {:.0}\n\
                Proj Radius: {:.1}\n\
                Fire Rate: {}\n\
                Magazine: {} rounds\n\
                Reload Delay: {:.2}s\n\
                Armor Pierce: {}\n\
                Detonation Range: {}\n\
                Submunitions: {}",
                barrel_count,
                mass,
                effective_mass,
                mass_desc,
                impact_damage,
                speed,
                radius,
                fire_rate_str,
                max_ammo,
                reload_delay,
                if can_pierce { "Yes" } else { "No" },
                aoe_str,
                split_str
            )
        }
        WeaponKind::Laser { .. } => "Laser stats (Incomplete)".to_string(),
    }
}

// =========================================================================
// 5. HANGAR MENU SETUP SYSTEM
// =========================================================================

pub fn setup_hangar_menu(
    mut commands: Commands,
    gear_state: Res<SelectedGearState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let turret_stats = format_weapon_stats(
        gear_state.turret_weapon,
        false,
        gear_state.turret_is_dual,
        &mut meshes,
        &mut materials,
    );

    let coaxial_stats = format_weapon_stats(
        gear_state.coaxial_weapon,
        true,
        gear_state.coaxial_is_dual,
        &mut meshes,
        &mut materials,
    );

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(30.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.08, 0.1)),
            DespawnOnExit(GameState::Hangar),
        ))
        .with_children(|root| {
            // Header Title
            root.spawn((
                Text::new("HANGAR LOADOUT BAY"),
                TextFont {
                    font_size: FontSize::Px(42.0),
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));

            // Main Columns Container
            root.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(75.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(30.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Stretch,
                ..default()
            })
            .with_children(|columns| {
                // LEFT COLUMN: TURRET SLOT
                spawn_slot_panel(
                    columns,
                    "TURRET MOUNT",
                    gear_state.turret_weapon.name(),
                    if gear_state.turret_is_dual {
                        "DUAL MOUNT"
                    } else {
                        "SINGLE MOUNT"
                    },
                    &turret_stats,
                    TurretMountToggleBtn,
                    TurretPrevBtn,
                    TurretNextBtn,
                    HangarUiLabel::TurretName,
                    HangarUiLabel::TurretMount,
                    HangarUiLabel::TurretStats,
                );

                // RIGHT COLUMN: COAXIAL SLOT
                spawn_slot_panel(
                    columns,
                    "COAXIAL MOUNT",
                    gear_state.coaxial_weapon.name(),
                    if gear_state.coaxial_is_dual {
                        "DUAL MOUNT"
                    } else {
                        "SINGLE MOUNT"
                    },
                    &coaxial_stats,
                    CoaxialMountToggleBtn,
                    CoaxialPrevBtn,
                    CoaxialNextBtn,
                    HangarUiLabel::CoaxialName,
                    HangarUiLabel::CoaxialMount,
                    HangarUiLabel::CoaxialStats,
                );
            });

            // Bottom Control Buttons
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(20.0),
                ..default()
            })
            .with_children(|footer| {
                footer
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(180.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                        HangarBackBtn,
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("MAIN MENU"),
                            TextFont {
                                font_size: FontSize::Px(22.0),
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });

                footer
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(240.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.1, 0.6, 0.2)),
                        LaunchMissionBtn,
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("LAUNCH MISSION"),
                            TextFont {
                                font_size: FontSize::Px(24.0),
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            });
        });
}

fn spawn_slot_panel(
    parent: &mut ChildSpawnerCommands,
    title: &str,
    weapon_name: &str,
    mount_name: &str,
    stats_text: &str,
    mount_btn_marker: impl Component,
    prev_btn_marker: impl Component,
    next_btn_marker: impl Component,
    name_label: HangarUiLabel,
    mount_label: HangarUiLabel,
    stats_label: HangarUiLabel,
) {
    parent
        .spawn((
            Node {
                width: Val::Px(450.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                row_gap: Val::Px(12.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.18)),
        ))
        .with_children(|panel| {
            panel.spawn((
                Text::new(title),
                TextFont {
                    font_size: FontSize::Px(28.0),
                    ..default()
                },
                TextColor(Color::srgb(0.2, 0.8, 1.0)),
            ));

            panel
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(220.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.25, 0.35, 0.45)),
                    mount_btn_marker,
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new(mount_name),
                        TextFont {
                            font_size: FontSize::Px(18.0),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        mount_label,
                    ));
                });

            panel
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(10.0),
                    ..default()
                })
                .with_children(|bar| {
                    bar.spawn((
                        Button,
                        Node {
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                        prev_btn_marker,
                    ))
                    .with_children(|b| {
                        b.spawn((
                            Text::new("<"),
                            TextFont {
                                font_size: FontSize::Px(20.0),
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });

                    bar.spawn((
                        Text::new(weapon_name),
                        TextFont {
                            font_size: FontSize::Px(22.0),
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.8, 0.2)),
                        name_label,
                    ));

                    bar.spawn((
                        Button,
                        Node {
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                        next_btn_marker,
                    ))
                    .with_children(|b| {
                        b.spawn((
                            Text::new(">"),
                            TextFont {
                                font_size: FontSize::Px(20.0),
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
                });

            panel
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_grow: 1.0,
                        padding: UiRect::all(Val::Px(12.0)),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.1, 0.1, 0.12)),
                ))
                .with_children(|box_node| {
                    box_node.spawn((
                        Text::new(stats_text),
                        TextFont {
                            font_size: FontSize::Px(15.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.85, 0.85, 0.85)),
                        stats_label,
                    ));
                });
        });
}

// =========================================================================
// 6. INTERACTION & LOADOUT COMPILATION SYSTEMS
// =========================================================================

pub fn handle_hangar_interactions(
    mut gear_state: ResMut<SelectedGearState>,
    turret_mount_q: Query<&Interaction, (Changed<Interaction>, With<TurretMountToggleBtn>)>,
    turret_prev_q: Query<&Interaction, (Changed<Interaction>, With<TurretPrevBtn>)>,
    turret_next_q: Query<&Interaction, (Changed<Interaction>, With<TurretNextBtn>)>,
    coaxial_mount_q: Query<&Interaction, (Changed<Interaction>, With<CoaxialMountToggleBtn>)>,
    coaxial_prev_q: Query<&Interaction, (Changed<Interaction>, With<CoaxialPrevBtn>)>,
    coaxial_next_q: Query<&Interaction, (Changed<Interaction>, With<CoaxialNextBtn>)>,
) {
    for interaction in &turret_mount_q {
        if *interaction == Interaction::Pressed {
            gear_state.turret_is_dual = !gear_state.turret_is_dual;
        }
    }
    for interaction in &turret_prev_q {
        if *interaction == Interaction::Pressed {
            gear_state.turret_weapon = gear_state.turret_weapon.prev();
        }
    }
    for interaction in &turret_next_q {
        if *interaction == Interaction::Pressed {
            gear_state.turret_weapon = gear_state.turret_weapon.next();
        }
    }

    for interaction in &coaxial_mount_q {
        if *interaction == Interaction::Pressed {
            gear_state.coaxial_is_dual = !gear_state.coaxial_is_dual;
        }
    }
    for interaction in &coaxial_prev_q {
        if *interaction == Interaction::Pressed {
            gear_state.coaxial_weapon = gear_state.coaxial_weapon.prev();
        }
    }
    for interaction in &coaxial_next_q {
        if *interaction == Interaction::Pressed {
            gear_state.coaxial_weapon = gear_state.coaxial_weapon.next();
        }
    }
}

pub fn update_hangar_ui(
    gear_state: Res<SelectedGearState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut text_q: Query<(&mut Text, &HangarUiLabel)>,
) {
    if !gear_state.is_changed() {
        return;
    }

    let turret_stats = format_weapon_stats(
        gear_state.turret_weapon,
        false,
        gear_state.turret_is_dual,
        &mut meshes,
        &mut materials,
    );

    let coaxial_stats = format_weapon_stats(
        gear_state.coaxial_weapon,
        true,
        gear_state.coaxial_is_dual,
        &mut meshes,
        &mut materials,
    );

    for (mut text, label) in &mut text_q {
        match label {
            HangarUiLabel::TurretName => text.0 = gear_state.turret_weapon.name().to_string(),
            HangarUiLabel::TurretMount => {
                text.0 = if gear_state.turret_is_dual {
                    "DUAL MOUNT"
                } else {
                    "SINGLE MOUNT"
                }
                .to_string();
            }
            HangarUiLabel::TurretStats => text.0 = turret_stats.clone(),
            HangarUiLabel::CoaxialName => text.0 = gear_state.coaxial_weapon.name().to_string(),
            HangarUiLabel::CoaxialMount => {
                text.0 = if gear_state.coaxial_is_dual {
                    "DUAL MOUNT"
                } else {
                    "SINGLE MOUNT"
                }
                .to_string();
            }
            HangarUiLabel::CoaxialStats => text.0 = coaxial_stats.clone(),
        }
    }
}

pub fn handle_hangar_buttons(
    mut commands: Commands,
    gear_state: Res<SelectedGearState>,
    mut loadout: ResMut<PlayerLoadout>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut next_state: ResMut<NextState<GameState>>,
    launch_q: Query<&Interaction, (Changed<Interaction>, With<LaunchMissionBtn>)>,
    back_q: Query<&Interaction, (Changed<Interaction>, With<HangarBackBtn>)>,
) {
    for interaction in &back_q {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::MainMenu);
        }
    }

    for interaction in &launch_q {
        if *interaction == Interaction::Pressed {
            let barrel_mat = materials.add(Color::srgb(0.1, 0.1, 0.1));

            // Compile Turret Weapon
            let turret_mount = if gear_state.turret_is_dual {
                MountPoint::DualTurret {
                    offset: Vec3::new(0.0, 30.0, -0.1),
                    spacing: 16.0,
                }
            } else {
                MountPoint::Turret {
                    offset: Vec3::new(0.0, 30.0, -0.1),
                }
            };

            loadout.turret_mounts = vec![MountedWeapon {
                weapon: gear_state.turret_weapon.build(&mut meshes, &mut materials),
                mount_point: turret_mount,
                mesh: gear_state.turret_weapon.barrel_mesh(&mut meshes),
                material: barrel_mat.clone(),
            }];

            // Compile Coaxial Weapon
            let coaxial_mount = if gear_state.coaxial_is_dual {
                MountPoint::DualCoaxial {
                    offset: Vec3::new(20.0, 40.0, -0.1),
                    spacing: 14.0,
                }
            } else {
                MountPoint::Coaxial {
                    offset: Vec3::new(20.0, 40.0, -0.1),
                }
            };

            loadout.coaxial_mounts = vec![MountedWeapon {
                weapon: gear_state.coaxial_weapon.build(&mut meshes, &mut materials),
                mount_point: coaxial_mount,
                mesh: gear_state.coaxial_weapon.barrel_mesh(&mut meshes),
                material: barrel_mat,
            }];

            // Bounce through MainMenu so OnTransition(MainMenu -> InGame) fires initialization
            commands.insert_resource(PendingRestart);
            next_state.set(GameState::MainMenu);
        }
    }
}
