use bevy::prelude::*;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    Paused,
}

#[derive(Component)]
struct PlayButton;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                handle_play_button_click.run_if(in_state(GameState::MainMenu)),
            );
    }
}

fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            DespawnOnExit(GameState::MainMenu),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
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
        });
}

fn handle_play_button_click(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(GameState::InGame);
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
