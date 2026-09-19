pub mod definitions;
pub mod hud;

use crate::ui::hud::{setup_hud, sync_weapon_cards, update_blink_ui, update_weapon_ui};
use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hud).add_systems(
            Update,
            (sync_weapon_cards, update_blink_ui, update_weapon_ui),
        );
    }
}
