use bevy::prelude::*;

/// Generic spawner function pointer signature for any enemy entity creator.
pub type EnemySpawnerFn = fn(
    &mut Commands,
    &mut ResMut<Assets<Mesh>>,
    &mut ResMut<Assets<ColorMaterial>>,
    Vec2,
) -> Entity;

/// Defines a group of a specific enemy type and how many to spawn.
#[derive(Clone)]
pub struct WaveEnemyGroup {
    pub spawner: EnemySpawnerFn,
    pub count: usize,
}

/// Declarative structure defining a wave configuration.
pub struct WaveDefinition {
    pub wave_number: u32,
    pub groups: Vec<WaveEnemyGroup>,
    /// Interval in seconds between trickle spawn bursts
    pub spawn_interval: f32,
    /// Number of enemies spawned per interval tick
    pub batch_size: usize,
}

/// Primary resource tracking global wave stats and internal spawn queue.
#[derive(Resource)]
pub struct WaveState {
    // --- Public Stats ---
    pub current_wave: u32,
    pub current_enemies: usize,
    pub total_enemies_killed: u64,
    /// Total time elapsed in seconds since the wave system started
    pub game_timer: f32,

    // --- Internal Wave Tracking ---
    pub wave_in_progress: bool,
    pub pending_spawns: Vec<EnemySpawnerFn>,
    pub spawn_timer: Timer,
    pub batch_size: usize,
}

impl Default for WaveState {
    fn default() -> Self {
        Self {
            current_wave: 0,
            current_enemies: 0,
            total_enemies_killed: 0,
            game_timer: 0.0,
            wave_in_progress: false,
            pending_spawns: Vec::new(),
            spawn_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            batch_size: 5,
        }
    }
}

impl WaveState {
    /// Queue up a new wave definition to begin spawning.
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
