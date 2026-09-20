use bevy::app::AppExit;
use bevy::prelude::*;

use super::GameState;
use crate::wave::definitions::WaveState;

#[derive(Resource)]
pub struct PendingRestart;

#[derive(Component)]
pub struct RetryButton;

#[derive(Component)]
pub struct GameOverMainMenuButton;

#[derive(Component)]
pub struct GameOverExitButton;

pub fn setup_game_over_menu(mut commands: Commands, wave_state: Option<Res<WaveState>>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(30.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.12, 0.02, 0.02, 0.88)),
            DespawnOnExit(GameState::GameOver),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("GAME OVER"),
                TextFont {
                    font_size: FontSize::Px(72.0),
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.15, 0.15)),
            ));

            if let Some(wave) = wave_state {
                parent
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(8.0),
                        ..default()
                    })
                    .with_children(|stats| {
                        let text_font = TextFont {
                            font_size: FontSize::Px(20.0),
                            ..default()
                        };
                        let text_color = TextColor(Color::srgb(0.8, 0.8, 0.8));

                        stats.spawn((
                            Text::new(format!("WAVE REACHED: {}", wave.current_wave)),
                            text_font.clone(),
                            text_color,
                        ));
                        stats.spawn((
                            Text::new(format!("ENEMIES DESTROYED: {}", wave.total_enemies_killed)),
                            text_font.clone(),
                            text_color,
                        ));
                        stats.spawn((
                            Text::new(format!("TIME SURVIVED: {:.1}s", wave.game_timer)),
                            text_font,
                            text_color,
                        ));
                    });
            }

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(14.0),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|menu_column| {
                    spawn_menu_button(menu_column, "RETRY", RetryButton);
                    spawn_menu_button(menu_column, "MAIN MENU", GameOverMainMenuButton);
                    spawn_menu_button(menu_column, "EXIT GAME", GameOverExitButton);
                });
        });
}

fn spawn_menu_button(parent: &mut ChildSpawnerCommands, label: &str, marker: impl Component) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(220.0),
                height: Val::Px(55.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            marker,
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(28.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

pub fn handle_retry_button_click(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<RetryButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Bounce to MainMenu first to trigger all DespawnOnEnter cleanups
                commands.insert_resource(PendingRestart);
                next_state.set(GameState::MainMenu);
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgb(0.35, 0.35, 0.35));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
            }
        }
    }
}

pub fn handle_main_menu_button_click(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<GameOverMainMenuButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(GameState::MainMenu);
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgb(0.35, 0.35, 0.35));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
            }
        }
    }
}

pub fn handle_exit_button_click(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<GameOverExitButton>),
    >,
    mut app_exit: MessageWriter<AppExit>,
) {
    for (interaction, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                app_exit.write(AppExit::Success);
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgb(0.35, 0.35, 0.35));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
            }
        }
    }
}
