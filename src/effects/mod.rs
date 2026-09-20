pub mod definitions;
pub mod systems;

use bevy::prelude::*;
use systems::*;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_effect_assets)
            .add_systems(Update, (update_particle_effects, update_shockwave_rings));
    }
}
