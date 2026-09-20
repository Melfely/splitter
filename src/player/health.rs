use crate::GameState;
use crate::effects::definitions::EffectAssets;
use crate::effects::systems::spawn_death_explosion_fx;
use crate::player::Player;
use crate::splitter_core::projectile::Durability;
use bevy::prelude::*;

pub fn player_health_system(
    mut commands: Commands,
    fx_assets: Res<EffectAssets>,
    mut next_state: ResMut<NextState<GameState>>,
    player_query: Query<(&GlobalTransform, &Durability), With<Player>>,
) {
    let Ok((global_tf, durability)) = player_query.single() else {
        return;
    };

    if durability.hp <= 0.0 {
        let position = global_tf.translation().truncate();

        // Spawn hull explosion particles
        spawn_death_explosion_fx(&mut commands, &fx_assets, position, true);

        // Transition to GameOver
        next_state.set(GameState::GameOver);
    }
}
