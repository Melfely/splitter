use crate::effects::definitions::EffectAssets;
use crate::effects::systems::spawn_shield_break_fx;
use crate::physics::definitions::{Collider, CollisionLayer};
use crate::player::definitions::{Player, PlayerShield, Shield, ShieldState};
use crate::splitter_core::projectile::Durability;
use bevy::prelude::*;

impl Default for Shield {
    fn default() -> Self {
        Self {
            state: ShieldState::Active,
            regen_rate: 15.0,
            recharge_delay: 3.0,
            delay_timer: Timer::from_seconds(3.0, TimerMode::Once),
            last_hp: 50.0,
        }
    }
}

pub fn update_shield_system(
    time: Res<Time>,
    fx_assets: Res<EffectAssets>,
    mut commands: Commands,
    mut shield_query: Query<(&GlobalTransform, &mut Shield, &mut Durability), With<PlayerShield>>,
) {
    let Ok((global_tf, mut shield, mut durability)) = shield_query.single_mut() else {
        return;
    };

    // 1. Detect if damage was taken this tick to reset recharge delay timer
    if durability.hp < shield.last_hp {
        shield.delay_timer.reset();
    }
    shield.last_hp = durability.hp;

    // 2. Manage state transitions, FX triggers, and regeneration
    match shield.state {
        ShieldState::Active => {
            if durability.hp <= 0.0 {
                durability.hp = 0.0;
                shield.state = ShieldState::Disabled;
                shield.delay_timer.reset();

                // Trigger cyan shield collapse shockwave + shards at exact shield position
                let position = global_tf.translation().truncate();
                spawn_shield_break_fx(&mut commands, &fx_assets, position);
            } else if durability.hp < durability.max_hp {
                shield.delay_timer.tick(time.delta());
                if shield.delay_timer.is_finished() {
                    durability.hp = (durability.hp + shield.regen_rate * time.delta_secs())
                        .min(durability.max_hp);
                }
            }
        }
        ShieldState::Disabled => {
            shield.delay_timer.tick(time.delta());
            if shield.delay_timer.is_finished() {
                durability.hp += shield.regen_rate * time.delta_secs();
                if durability.hp >= durability.max_hp {
                    durability.hp = durability.max_hp;
                    shield.state = ShieldState::Active;
                }
            }
        }
    }
}

pub fn sync_shield_collider_and_visibility(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut shield_query: Query<
        (Entity, &Shield, &mut Visibility),
        (With<PlayerShield>, Changed<Shield>),
    >,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };
    let Ok((shield_entity, shield, mut visibility)) = shield_query.single_mut() else {
        return;
    };

    match shield.state {
        ShieldState::Active => {
            *visibility = Visibility::Inherited;

            commands.entity(shield_entity).insert(Collider {
                radius: 85.0,
                layer: CollisionLayer::Player,
            });

            commands.entity(player_entity).remove::<Collider>();
        }
        ShieldState::Disabled => {
            *visibility = Visibility::Hidden;

            commands.entity(shield_entity).remove::<Collider>();

            commands.entity(player_entity).insert(Collider {
                radius: 40.0,
                layer: CollisionLayer::Player,
            });
        }
    }
}
