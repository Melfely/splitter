use bevy::prelude::*;
use std::collections::HashMap;

use crate::player::definitions::{PlayerBlink, PlayerPhysics, Shield};
use crate::splitter_core::turret::Turret;
use crate::splitter_core::weapon::Weapon;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CardType {
    FireRate,
    ProjectileSpeed,
    ReloadSpeed,
    Multishot,
    TurretSpeed,
    EngineAcceleration,
    BlinkCooldown,
    ShieldRegen,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CardId {
    pub card_type: CardType,
    pub target_entity: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct CardHistory {
    pub counts: HashMap<CardId, u32>,
}

impl CardHistory {
    pub fn get_count(&self, id: &CardId) -> u32 {
        *self.counts.get(id).unwrap_or(&0)
    }

    pub fn increment(&mut self, id: &CardId) {
        *self.counts.entry(id.clone()).or_insert(0) += 1;
    }

    /// Linear factor for multiplicative stat increases (+p per tier off base).
    pub fn linear_increase_factor(&self, id: &CardId, step_p: f32) -> f32 {
        let n = self.get_count(id) as f32;
        (1.0 + (n + 1.0) * step_p) / (1.0 + n * step_p)
    }

    /// Linear factor for multiplicative stat decreases (-p per tier off base).
    pub fn linear_decrease_factor(&self, id: &CardId, step_p: f32) -> f32 {
        let n = self.get_count(id) as f32;
        (1.0 - (n + 1.0) * step_p) / (1.0 - n * step_p)
    }
}

pub trait Card: Send + Sync {
    fn id(&self) -> CardId;
    fn title(&self) -> String;
    fn description(&self) -> String;
    fn apply(
        &self,
        history: &mut CardHistory,
        physics: &mut Query<&mut PlayerPhysics>,
        blink: &mut Query<&mut PlayerBlink>,
        shield: &mut Query<&mut Shield>,
        turrets: &mut Query<&mut Turret>,
        weapons: &mut Query<(Entity, &mut Weapon)>,
    );
}

pub type EnemySpawnerFn = fn(
    &mut Commands,
    &mut ResMut<Assets<Mesh>>,
    &mut ResMut<Assets<ColorMaterial>>,
    Vec2,
) -> Entity;

#[derive(Clone)]
pub struct WaveEnemyGroup {
    pub spawner: EnemySpawnerFn,
    pub count: usize,
}

pub struct WaveDefinition {
    pub wave_number: u32,
    pub groups: Vec<WaveEnemyGroup>,
    pub spawn_interval: f32,
    pub batch_size: usize,
}

#[derive(Resource, Default)]
pub struct WaveState {
    pub current_wave: u32,
    pub current_enemies: usize,
    pub total_enemies_killed: u64,
    pub game_timer: f32,

    pub wave_in_progress: bool,
    pub pending_spawns: Vec<EnemySpawnerFn>,
    pub spawn_timer: Timer,
    pub batch_size: usize,
}

impl WaveState {
    pub fn start_wave(&mut self, definition: WaveDefinition) {
        self.current_wave = definition.wave_number;
        self.wave_in_progress = true;
        self.batch_size = definition.batch_size;
        self.spawn_timer = Timer::from_seconds(definition.spawn_interval, TimerMode::Repeating);

        self.pending_spawns.clear();
        for group in definition.groups {
            for _ in 0..group.count {
                self.pending_spawns.push(group.spawner);
            }
        }
    }
}
