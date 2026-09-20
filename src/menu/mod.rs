use crate::player::definitions::PlayerLoadout;
use bevy::prelude::*;

pub mod card_selector;
pub mod game_over;
pub mod hangar;
pub mod main;
pub mod pause;

use game_over::PendingRestart;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Hangar,
    InGame,
    CardSelect,
    Paused,
    GameOver,
}

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
            .init_resource::<hangar::SelectedGearState>()
            .init_resource::<PlayerLoadout>()
            .init_resource::<crate::wave::card::ActiveCardSelection>()
            // Main Menu Systems
            .add_systems(
                OnEnter(GameState::MainMenu),
                (handle_pending_restart, main::setup_main_menu),
            )
            .add_systems(
                Update,
                (
                    main::handle_play_button_click,
                    main::handle_hangar_button_click,
                    main::handle_exit_button_click,
                )
                    .run_if(in_state(GameState::MainMenu)),
            )
            // Hangar / Gear Selection Systems
            .add_systems(OnEnter(GameState::Hangar), hangar::setup_hangar_menu)
            .add_systems(
                Update,
                (
                    hangar::handle_hangar_interactions,
                    hangar::update_hangar_ui,
                    hangar::handle_hangar_buttons,
                )
                    .run_if(in_state(GameState::Hangar)),
            )
            // Card Selection Systems
            .add_systems(
                OnEnter(GameState::CardSelect),
                card_selector::setup_card_selector,
            )
            .add_systems(
                Update,
                (
                    card_selector::handle_card_interactions,
                    card_selector::handle_card_key_inputs,
                )
                    .run_if(in_state(GameState::CardSelect)),
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
