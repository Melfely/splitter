use bevy::prelude::*;

use crate::menu::GameState;
use crate::player::definitions::{CoaxialWeapon, MainWeapon, PlayerBlink, PlayerPhysics, Shield};
use crate::splitter_core::turret::Turret;
use crate::splitter_core::weapon::Weapon;
use crate::wave::card::{ActiveCardSelection, WeaponInfo};
use crate::wave::definitions::CardHistory;

#[derive(Component)]
pub struct CardButton {
    pub index: usize,
}

pub fn setup_card_selector(
    mut commands: Commands,
    mut selection: ResMut<ActiveCardSelection>,
    weapon_query: Query<(
        Entity,
        &Weapon,
        Option<&Name>,
        Option<&MainWeapon>,
        Option<&CoaxialWeapon>,
    )>,
) {
    let equipped_weapons = WeaponInfo::from_query(&weapon_query);
    selection.generate_selection(&equipped_weapons);
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(Color::srgba(0.02, 0.02, 0.05, 0.88)),
            DespawnOnExit(GameState::CardSelect),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("WAVE COMPLETED — CHOOSE AN UPGRADE"),
                TextFont {
                    font_size: FontSize::Px(32.0),
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(20.0),
                    ..default()
                })
                .with_children(|row| {
                    for (i, card) in selection.options.iter().enumerate() {
                        row.spawn((
                            Button,
                            CardButton { index: i },
                            Node {
                                width: Val::Px(240.0),
                                height: Val::Px(320.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(Val::Px(20.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BorderColor::all(Color::srgb(0.3, 0.5, 0.8)),
                            BackgroundColor(Color::srgba(0.08, 0.12, 0.2, 0.95)),
                        ))
                        .with_children(|card_node| {
                            card_node.spawn((
                                Text::new(format!("[ {} ]", i + 1)),
                                TextFont {
                                    font_size: FontSize::Px(20.0),
                                    ..default()
                                },
                                TextColor(Color::srgb(0.4, 0.8, 1.0)),
                            ));

                            card_node.spawn((
                                Text::new(card.title()),
                                TextFont {
                                    font_size: FontSize::Px(18.0),
                                    ..default()
                                },
                                TextColor(Color::srgb(1.0, 0.85, 0.3)),
                                Node {
                                    margin: UiRect::vertical(Val::Px(10.0)),
                                    ..default()
                                },
                            ));

                            card_node.spawn((
                                Text::new(card.description()),
                                TextFont {
                                    font_size: FontSize::Px(14.0),
                                    ..default()
                                },
                                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                            ));

                            card_node.spawn((
                                Text::new("CLICK OR PRESS KEY"),
                                TextFont {
                                    font_size: FontSize::Px(12.0),
                                    ..default()
                                },
                                TextColor(Color::srgb(0.5, 0.5, 0.5)),
                            ));
                        });
                    }
                });
        });
}

pub fn handle_card_interactions(
    selection: Res<ActiveCardSelection>,
    mut history: ResMut<CardHistory>,
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &CardButton, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut physics: Query<&mut PlayerPhysics>,
    mut blink: Query<&mut PlayerBlink>,
    mut shield: Query<&mut Shield>,
    mut turrets: Query<&mut Turret>,
    mut weapons: Query<(Entity, &mut Weapon)>,
) {
    for (interaction, card_btn, mut border) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                apply_selected_card(
                    card_btn.index,
                    &selection,
                    &mut history,
                    &mut next_state,
                    &mut physics,
                    &mut blink,
                    &mut shield,
                    &mut turrets,
                    &mut weapons,
                );
            }
            Interaction::Hovered => {
                *border = BorderColor::all(Color::srgb(0.9, 0.7, 0.2));
            }
            Interaction::None => {
                *border = BorderColor::all(Color::srgb(0.3, 0.5, 0.8));
            }
        }
    }
}

pub fn handle_card_key_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    selection: Res<ActiveCardSelection>,
    mut history: ResMut<CardHistory>,
    mut next_state: ResMut<NextState<GameState>>,
    mut physics: Query<&mut PlayerPhysics>,
    mut blink: Query<&mut PlayerBlink>,
    mut shield: Query<&mut Shield>,
    mut turrets: Query<&mut Turret>,
    mut weapons: Query<(Entity, &mut Weapon)>,
) {
    let index = if keys.just_pressed(KeyCode::Digit1) || keys.just_pressed(KeyCode::Numpad1) {
        Some(0)
    } else if keys.just_pressed(KeyCode::Digit2) || keys.just_pressed(KeyCode::Numpad2) {
        Some(1)
    } else if keys.just_pressed(KeyCode::Digit3) || keys.just_pressed(KeyCode::Numpad3) {
        Some(2)
    } else {
        None
    };

    if let Some(i) = index {
        apply_selected_card(
            i,
            &selection,
            &mut history,
            &mut next_state,
            &mut physics,
            &mut blink,
            &mut shield,
            &mut turrets,
            &mut weapons,
        );
    }
}

fn apply_selected_card(
    index: usize,
    selection: &ActiveCardSelection,
    history: &mut CardHistory,
    next_state: &mut ResMut<NextState<GameState>>,
    physics: &mut Query<&mut PlayerPhysics>,
    blink: &mut Query<&mut PlayerBlink>,
    shield: &mut Query<&mut Shield>,
    turrets: &mut Query<&mut Turret>,
    weapons: &mut Query<(Entity, &mut Weapon)>,
) {
    if let Some(card) = selection.options.get(index) {
        card.apply(history, physics, blink, shield, turrets, weapons);
        next_state.set(GameState::InGame);
    }
}
