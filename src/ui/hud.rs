use crate::menu::GameState;
use crate::splitter_core::weapon::{ProjectileState, Weapon, WeaponKind};
use bevy::prelude::*;
use std::collections::HashSet; // References existing weapon structures[cite: 2]

use crate::player::definitions::{Player, PlayerBlink};
use crate::player::shield::PlayerShield;
use crate::splitter_core::projectile::Durability;

use crate::ui::definitions::*;

pub fn setup_hud(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..default()
            },
            // Automatically cleans up the entire HUD tree when returning to Main Menu
            DespawnOnEnter(GameState::MainMenu),
        ))
        .with_children(|root| {
            // TOP-LEFT WAVE STATS PANEL
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(16.0),
                    left: Val::Px(16.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.85)),
                BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
            ))
            .with_children(|wave_panel| {
                wave_panel.spawn((
                    Text::new("WAVE: 0"),
                    TextFont {
                        font_size: FontSize::Px(15.0),
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.8, 0.2)),
                    UiWaveText,
                ));

                wave_panel.spawn((
                    Text::new("ENEMIES: 0"),
                    TextFont {
                        font_size: FontSize::Px(13.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    UiEnemiesText,
                ));

                wave_panel.spawn((
                    Text::new("KILLS: 0"),
                    TextFont {
                        font_size: FontSize::Px(13.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    UiKillsText,
                ));

                wave_panel.spawn((
                    Text::new("TIME: 00:00"),
                    TextFont {
                        font_size: FontSize::Px(13.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    UiGameTimerText,
                ));
            });

            // TOP-CENTER HUD: SHIELD & HULL BARS
            root.spawn(Node {
                position_type: PositionType::Absolute,
                top: Val::Px(16.0),
                left: Val::Percent(0.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(6.0),
                ..default()
            })
            .with_children(|center_panel| {
                // SHIELD BAR
                center_panel
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        ..default()
                    })
                    .with_children(|row| {
                        row.spawn((
                            Text::new("SHIELD"),
                            TextFont {
                                font_size: FontSize::Px(12.0),
                                ..default()
                            },
                            TextColor(Color::srgb(0.2, 0.8, 1.0)),
                        ));

                        row.spawn((
                            Node {
                                width: Val::Px(240.0),
                                height: Val::Px(12.0),
                                border: UiRect::all(Val::Px(1.0)),
                                padding: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.85)),
                            BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
                        ))
                        .with_children(|track| {
                            track.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.2, 0.8, 1.0)),
                                UiShieldBar,
                            ));
                        });
                    });

                // HULL (HP) BAR
                center_panel
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        ..default()
                    })
                    .with_children(|row| {
                        row.spawn((
                            Text::new("HULL  "),
                            TextFont {
                                font_size: FontSize::Px(12.0),
                                ..default()
                            },
                            TextColor(Color::srgb(0.2, 0.9, 0.3)),
                        ));

                        row.spawn((
                            Node {
                                width: Val::Px(240.0),
                                height: Val::Px(12.0),
                                border: UiRect::all(Val::Px(1.0)),
                                padding: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.85)),
                            BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
                        ))
                        .with_children(|track| {
                            track.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.2, 0.9, 0.3)),
                                UiHealthBar,
                            ));
                        });
                    });
            });

            // TOP-RIGHT FPS COUNTER
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(16.0),
                    right: Val::Px(16.0),
                    padding: UiRect::axes(Val::Px(10.0), Val::Px(6.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.85)),
                BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
            ))
            .with_children(|fps_panel| {
                fps_panel.spawn((
                    Text::new("FPS: --"),
                    TextFont {
                        font_size: FontSize::Px(13.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.0, 0.9, 0.4)),
                    UiFpsText,
                ));
            });

            // BOTTOM HUD (Blink UI & Weapon Slots)
            root.spawn(Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(16.0),
                left: Val::Px(16.0),
                right: Val::Px(16.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexEnd,
                ..default()
            })
            .with_children(|bottom_bar| {
                spawn_blink_ui(bottom_bar);

                bottom_bar.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(12.0),
                        ..default()
                    },
                    UiWeaponContainer,
                ));
            });
        });
}

// Automatically creates UI cards when new Weapon entities exist
pub fn sync_weapon_cards(
    mut commands: Commands,
    container_query: Query<Entity, With<UiWeaponContainer>>,
    weapon_query: Query<Entity, With<Weapon>>,
    card_query: Query<&WeaponUiCard>,
) {
    let Ok(container_entity) = container_query.single() else {
        return;
    };

    let existing_cards: HashSet<Entity> =
        card_query.iter().map(|card| card.target_weapon).collect();

    for weapon_entity in weapon_query.iter() {
        if !existing_cards.contains(&weapon_entity) {
            commands.entity(container_entity).with_children(|parent| {
                spawn_weapon_card(parent, weapon_entity);
            });
        }
    }
}

fn spawn_blink_ui(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.85)),
            BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
        ))
        .with_children(|panel| {
            panel.spawn((
                Text::new("STRAFE BLINK"),
                TextFont {
                    font_size: FontSize::Px(12.0),
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));

            spawn_cooldown_row(panel, "Q [LEFT]", UiBlinkBarQ);
            spawn_cooldown_row(panel, "E [RIGHT]", UiBlinkBarE);
        });
}

fn spawn_cooldown_row(parent: &mut ChildSpawnerCommands, label: &str, marker: impl Component) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(8.0),
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            row.spawn((
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(10.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            ))
            .with_children(|bar_bg| {
                bar_bg.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.0, 0.9, 0.4)),
                    marker,
                ));
            });
        });
}

fn spawn_weapon_card(parent: &mut ChildSpawnerCommands, weapon_entity: Entity) {
    parent
        .spawn((
            WeaponUiCard {
                target_weapon: weapon_entity,
            },
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Px(160.0),
                padding: UiRect::all(Val::Px(10.0)),
                row_gap: Val::Px(6.0),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.08, 0.08, 0.08, 0.9)),
            BorderColor::all(Color::srgb(0.4, 0.4, 0.4)),
        ))
        .with_children(|card| {
            card.spawn((
                Text::new("WEAPON SLOT"),
                TextFont {
                    font_size: FontSize::Px(13.0),
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));

            card.spawn((
                Text::new("AMMO: --/--"),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(Color::WHITE),
                UiAmmoText,
            ));

            card.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(8.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            ))
            .with_children(|bar_bg| {
                bar_bg.spawn((
                    Node {
                        width: Val::Percent(0.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(1.0, 0.3, 0.0)),
                    UiHeatBarInner,
                ));
            });

            card.spawn((
                Text::new("READY"),
                TextFont {
                    font_size: FontSize::Px(11.0),
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
                UiLaserStatusText,
            ));
        });
}

pub fn update_weapon_ui(
    weapons: Query<&Weapon>,
    cards: Query<(&WeaponUiCard, &Children)>,
    mut ammo_texts: Query<&mut Text, (With<UiAmmoText>, Without<UiLaserStatusText>)>,
    mut status_texts: Query<&mut Text, (With<UiLaserStatusText>, Without<UiAmmoText>)>,
    mut heat_bars: Query<&mut Node, With<UiHeatBarInner>>,
) {
    for (card, children) in cards.iter() {
        let Ok(weapon) = weapons.get(card.target_weapon) else {
            continue;
        };

        for child in children.iter() {
            match &weapon.kind {
                WeaponKind::Projectile {
                    current_ammo,
                    max_ammo,
                    state,
                    ..
                } => {
                    if let Ok(mut text) = ammo_texts.get_mut(child) {
                        match state {
                            ProjectileState::Reloading(timer) => {
                                let pct = (timer.fraction() * 100.0) as u32;
                                **text = format!("RELOADING ({pct}%)");
                            }
                            _ => {
                                **text = format!("AMMO: {current_ammo}/{max_ammo}");
                            }
                        }
                    }
                    if let Ok(mut bar) = heat_bars.get_mut(child) {
                        bar.width = Val::Percent(0.0);
                    }
                    if let Ok(mut status) = status_texts.get_mut(child) {
                        if status.is_empty() {
                            **status = "PROJECTILE".to_string();
                        }
                    }
                }
                WeaponKind::Laser {
                    current_heat,
                    max_heat,
                    is_recharging,
                    ..
                } => {
                    if let Ok(mut text) = ammo_texts.get_mut(child) {
                        let pct = ((current_heat / max_heat) * 100.0) as u32;
                        **text = format!("HEAT: {pct}%");
                    }
                    if let Ok(mut bar) = heat_bars.get_mut(child) {
                        let fill = (current_heat / max_heat).clamp(0.0, 1.0) * 100.0;
                        bar.width = Val::Percent(fill);
                    }
                    if let Ok(mut status) = status_texts.get_mut(child) {
                        if *is_recharging {
                            **status = "OVERHEATED".to_string();
                        } else {
                            **status = "LASER READY".to_string();
                        }
                    }
                }
            }
        }
    }
}

pub fn update_blink_ui(
    time: Res<Time>,
    mut blink_query: Query<&mut PlayerBlink, With<Player>>,
    mut q_bar: Query<&mut Node, (With<UiBlinkBarQ>, Without<UiBlinkBarE>)>,
    mut e_bar: Query<&mut Node, (With<UiBlinkBarE>, Without<UiBlinkBarQ>)>,
) {
    let Ok(mut blink) = blink_query.single_mut() else {
        return;
    };

    blink.q_cooldown.tick(time.delta());
    blink.e_cooldown.tick(time.delta());

    if let Ok(mut node) = q_bar.single_mut() {
        let pct = if blink.q_cooldown.is_finished() {
            100.0
        } else {
            blink.q_cooldown.fraction() * 100.0
        };
        node.width = Val::Percent(pct);
    }

    if let Ok(mut node) = e_bar.single_mut() {
        let pct = if blink.e_cooldown.is_finished() {
            100.0
        } else {
            blink.e_cooldown.fraction() * 100.0
        };
        node.width = Val::Percent(pct);
    }
}

// System to update FPS value smooth over 0.25s intervals
pub fn update_fps_ui(
    time: Res<Time>,
    mut accumulated_frames: Local<u32>,
    mut accumulated_time: Local<f32>,
    mut fps_texts: Query<&mut Text, With<UiFpsText>>,
) {
    let dt = time.delta_secs();
    *accumulated_time += dt;
    *accumulated_frames += 1;

    if *accumulated_time >= 0.25 {
        let fps = (*accumulated_frames as f32 / *accumulated_time).round() as u32;
        *accumulated_frames = 0;
        *accumulated_time = 0.0;

        for mut text in fps_texts.iter_mut() {
            **text = format!("FPS: {fps}");
        }
    }
}

use crate::wave::definitions::WaveState;

pub fn update_wave_ui(
    wave_state: Res<WaveState>,
    mut wave_texts: Query<
        &mut Text,
        (
            With<UiWaveText>,
            Without<UiEnemiesText>,
            Without<UiKillsText>,
            Without<UiGameTimerText>,
        ),
    >,
    mut enemies_texts: Query<
        &mut Text,
        (
            With<UiEnemiesText>,
            Without<UiWaveText>,
            Without<UiKillsText>,
            Without<UiGameTimerText>,
        ),
    >,
    mut kills_texts: Query<
        &mut Text,
        (
            With<UiKillsText>,
            Without<UiWaveText>,
            Without<UiEnemiesText>,
            Without<UiGameTimerText>,
        ),
    >,
    mut timer_texts: Query<
        &mut Text,
        (
            With<UiGameTimerText>,
            Without<UiWaveText>,
            Without<UiEnemiesText>,
            Without<UiKillsText>,
        ),
    >,
) {
    if !wave_state.is_changed() {
        return;
    }

    for mut text in wave_texts.iter_mut() {
        **text = format!("WAVE: {}", wave_state.current_wave);
    }

    for mut text in enemies_texts.iter_mut() {
        **text = format!("ENEMIES: {}", wave_state.current_enemies);
    }

    for mut text in kills_texts.iter_mut() {
        **text = format!("KILLS: {}", wave_state.total_enemies_killed);
    }

    for mut text in timer_texts.iter_mut() {
        let total_seconds = wave_state.game_timer as u32;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        **text = format!("TIME: {minutes:02}:{seconds:02}");
    }
}

pub fn update_player_hud_bars(
    player_query: Query<&Durability, (With<Player>, Without<PlayerShield>)>,
    shield_query: Query<&Durability, With<PlayerShield>>,
    mut shield_bar_query: Query<&mut Node, With<UiShieldBar>>,
    mut health_bar_query: Query<&mut Node, (With<UiHealthBar>, Without<UiShieldBar>)>,
) {
    if let Ok(shield_durability) = shield_query.single() {
        if let Ok(mut bar_node) = shield_bar_query.single_mut() {
            let pct = (shield_durability.hp / shield_durability.max_hp).clamp(0.0, 1.0) * 100.0;
            bar_node.width = Val::Percent(pct);
        }
    }

    if let Ok(player_durability) = player_query.single() {
        if let Ok(mut bar_node) = health_bar_query.single_mut() {
            let pct = (player_durability.hp / player_durability.max_hp).clamp(0.0, 1.0) * 100.0;
            bar_node.width = Val::Percent(pct);
        }
    }
}
