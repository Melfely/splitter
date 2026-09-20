use bevy::prelude::*;

pub mod game_over;
pub mod main;
pub mod pause;

use game_over::PendingRestart;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    Paused,
    GameOver,
}

/// If we bounced to MainMenu from Retry, immediately start a new run
fn handle_pending_restart(
    mut commands: Commands,
    pending: Option<Res<PendingRestart>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if pending.is_some() {
        commands.remove_resource::<PendingRestart>();
        next_state.set(GameState::InGame);
    }
}

pub struct SplitterMenuPlugin;

impl Plugin for SplitterMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            // Main Menu Systems
            .add_systems(
                OnEnter(GameState::MainMenu),
                (handle_pending_restart, main::setup_main_menu),
            )
            .add_systems(
                Update,
                (
                    main::handle_play_button_click,
                    main::handle_exit_button_click,
                )
                    .run_if(in_state(GameState::MainMenu)),
            )
            // Pause Systems
            .add_systems(
                Update,
                pause::toggle_pause_system
                    .run_if(in_state(GameState::InGame).or_else(in_state(GameState::Paused))),
            )
            .add_systems(OnEnter(GameState::Paused), pause::setup_pause_menu)
            .add_systems(
                Update,
                (
                    pause::handle_resume_button_click,
                    pause::handle_main_menu_button_click,
                    pause::handle_pause_exit_button_click,
                )
                    .run_if(in_state(GameState::Paused)),
            )
            // Game Over Systems
            .add_systems(
                OnEnter(GameState::GameOver),
                game_over::setup_game_over_menu,
            )
            .add_systems(
                Update,
                (
                    game_over::handle_retry_button_click,
                    game_over::handle_main_menu_button_click,
                    game_over::handle_exit_button_click,
                )
                    .run_if(in_state(GameState::GameOver)),
            );
    }
}
