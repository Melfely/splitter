use bevy::prelude::*;

#[derive(Component)]
pub struct Particle {
    pub lifetime: Timer,
    pub velocity: Vec2,
    pub angular_velocity: f32,
    pub initial_scale: Vec2,
    pub shrink: bool,
    pub fade: bool,
}

#[derive(Component)]
pub struct ShockwaveRing {
    pub lifetime: Timer,
    pub max_scale: f32,
}

#[derive(Resource)]
pub struct EffectAssets {
    pub particle_mesh: Handle<Mesh>,
    pub shockwave_mesh: Handle<Mesh>,
    pub shield_break_material: Handle<ColorMaterial>,
    pub player_death_material: Handle<ColorMaterial>,
    pub enemy_death_material: Handle<ColorMaterial>,
}
