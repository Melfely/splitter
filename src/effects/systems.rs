use super::definitions::{EffectAssets, Particle, ShockwaveRing};
use bevy::prelude::*;

/// Initializes shared meshes/materials used by FX systems to minimize runtime allocations
pub fn setup_effect_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let effect_assets = EffectAssets {
        particle_mesh: meshes.add(Rectangle::new(12.0, 12.0)),
        shockwave_mesh: meshes.add(Annulus::new(10.0, 14.0)),
        shield_break_material: materials.add(Color::srgba(0.0, 0.85, 1.0, 0.9)),
        player_death_material: materials.add(Color::srgba(0.9, 0.2, 0.1, 1.0)),
        enemy_death_material: materials.add(Color::srgba(0.2, 0.9, 0.3, 1.0)),
    };

    commands.insert_resource(effect_assets);
}

/// Spawns a radial particle shatter + expanding cyan shockwave on shield collapse
pub fn spawn_shield_break_fx(commands: &mut Commands, fx_assets: &EffectAssets, position: Vec2) {
    // 1. Expanding energy shockwave
    commands.spawn((
        Mesh2d(fx_assets.shockwave_mesh.clone()),
        MeshMaterial2d(fx_assets.shield_break_material.clone()),
        Transform::from_translation(position.extend(2.0)),
        ShockwaveRing {
            lifetime: Timer::from_seconds(0.35, TimerMode::Once),
            max_scale: 8.0,
        },
    ));

    // 2. High-speed crystalline shards
    let shard_count = 16;
    for i in 0..shard_count {
        let angle = (i as f32 / shard_count as f32) * std::f32::consts::TAU;
        let dir = Vec2::new(angle.cos(), angle.sin());
        let speed = rand::random::<f32>() * 350.0 + 200.0;

        commands.spawn((
            Mesh2d(fx_assets.particle_mesh.clone()),
            MeshMaterial2d(fx_assets.shield_break_material.clone()),
            Transform::from_translation(position.extend(2.5))
                .with_rotation(Quat::from_rotation_z(angle))
                .with_scale(Vec3::new(0.6, 1.8, 1.0)),
            Particle {
                lifetime: Timer::from_seconds(0.4, TimerMode::Once),
                velocity: dir * speed,
                angular_velocity: (rand::random::<f32>() - 0.5) * 15.0,
                initial_scale: Vec2::new(0.6, 1.8),
                shrink: true,
                fade: true,
            },
        ));
    }
}

/// Spawns an violent explosion of hull fragments when an entity dies
pub fn spawn_death_explosion_fx(
    commands: &mut Commands,
    fx_assets: &EffectAssets,
    position: Vec2,
    is_player: bool,
) {
    let mat = if is_player {
        fx_assets.player_death_material.clone()
    } else {
        fx_assets.enemy_death_material.clone()
    };

    let particle_count = if is_player { 32 } else { 18 };

    for _ in 0..particle_count {
        let angle = rand::random::<f32>() * std::f32::consts::TAU;
        let speed = rand::random::<f32>() * 500.0 + 100.0;
        let dir = Vec2::new(angle.cos(), angle.sin());
        let scale = Vec2::splat(rand::random::<f32>() * 1.2 + 0.4);

        commands.spawn((
            Mesh2d(fx_assets.particle_mesh.clone()),
            MeshMaterial2d(mat.clone()),
            Transform::from_translation(position.extend(3.0)).with_scale(scale.extend(1.0)),
            Particle {
                lifetime: Timer::from_seconds(rand::random::<f32>() * 0.5 + 0.3, TimerMode::Once),
                velocity: dir * speed,
                angular_velocity: (rand::random::<f32>() - 0.5) * 20.0,
                initial_scale: scale,
                shrink: true,
                fade: true,
            },
        ));
    }
}

/// Updates particle trajectories, rotations, and shrinks/fades them out over time
pub fn update_particle_effects(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Particle)>,
) {
    let delta = time.delta_secs();

    for (entity, mut transform, mut particle) in query.iter_mut() {
        particle.lifetime.tick(time.delta());

        if particle.lifetime.is_finished() {
            commands.entity(entity).despawn();
            continue;
        }

        let progress = particle.lifetime.fraction();
        let remaining_ratio = 1.0 - progress;

        // Kinematics
        transform.translation += (particle.velocity * delta).extend(0.0);
        transform.rotate_z(particle.angular_velocity * delta);

        // Linear drag on particles
        particle.velocity *= (-4.0 * delta).exp();

        // Shrink scaling
        if particle.shrink {
            let current_scale = particle.initial_scale * remaining_ratio;
            transform.scale = current_scale.extend(1.0);
        }
    }
}

/// Expands shockwave rings outward and despawns on completion
pub fn update_shockwave_rings(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut ShockwaveRing)>,
) {
    for (entity, mut transform, mut ring) in query.iter_mut() {
        ring.lifetime.tick(time.delta());

        if ring.lifetime.is_finished() {
            commands.entity(entity).despawn();
            continue;
        }

        let progress = ring.lifetime.fraction();
        let scale = 1.0 + (ring.max_scale - 1.0) * progress;
        transform.scale = Vec3::new(scale, scale, 1.0);
    }
}
