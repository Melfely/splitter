pub mod definitions;
pub mod hud;

use crate::ui::hud::{
    setup_hud, sync_weapon_cards, update_blink_ui, update_fps_ui, update_wave_ui, update_weapon_ui,
};
use crate::{GameState, GameplaySet};
use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_hud)
            .add_systems(
                Update,
                (
                    sync_weapon_cards,
                    update_blink_ui,
                    update_weapon_ui,
                    update_fps_ui,
                    update_wave_ui,
                )
                    .in_set(GameplaySet),
            );
    }
}
