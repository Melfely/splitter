use crate::menu::GameState;
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayButton;
#[derive(Component)]
pub struct HangarButton;
#[derive(Component)]
pub struct ExitButton;

pub fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(50.0),
                ..default()
            },
            DespawnOnExit(GameState::MainMenu),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("SPLITTER"),
                TextFont {
                    font_size: FontSize::Px(72.0),
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.0),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|menu_column| {
                    // PLAY BUTTON
                    menu_column
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(220.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.1, 0.6, 0.2)),
                            PlayButton,
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new("PLAY"),
                                TextFont {
                                    font_size: FontSize::Px(32.0),
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });

                    // HANGAR / GEAR BUTTON
                    menu_column
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(220.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.2, 0.35, 0.5)),
                            HangarButton,
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new("HANGAR"),
                                TextFont {
                                    font_size: FontSize::Px(32.0),
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });

                    // EXIT BUTTON
                    menu_column
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(220.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                            ExitButton,
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new("EXIT"),
                                TextFont {
                                    font_size: FontSize::Px(32.0),
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                });
        });
}

pub fn handle_play_button_click(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::InGame);
        }
    }
}

pub fn handle_hangar_button_click(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<HangarButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Hangar);
        }
    }
}

pub fn handle_exit_button_click(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
    mut app_exit: MessageWriter<AppExit>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            app_exit.write(AppExit::Success);
        }
    }
}
