use bevy::prelude::*;
use rand::RngExt;

use crate::splitter_core::Lifetime;

#[derive(Component)]
pub struct HitParticle {
    pub velocity: Vec2,
}

pub fn update_hit_particles(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &HitParticle, &Lifetime)>,
) {
    let dt = time.delta_secs();
    for (_entity, mut transform, particle, lifetime) in query.iter_mut() {
        // 1. Move particle by velocity
        transform.translation.x += particle.velocity.x * dt;
        transform.translation.y += particle.velocity.y * dt;

        // 2. Shrink particle as lifetime expires
        let scale = lifetime.0.fraction_remaining();
        transform.scale = Vec3::splat(scale);
    }
}

pub fn spawn_impact_sparks(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    impact_pos: Vec3,
    bullet_dir: Vec2,
    spark_color: Color,
) {
    let mut rng = rand::rng(); // Replaced thread_rng() with rng()
    let spark_mat = materials.add(spark_color);

    let base_angle = bullet_dir.y.atan2(bullet_dir.x);

    // Replaced gen_range() with random_range()
    let particle_count = rng.random_range(14..=22);

    for _ in 0..particle_count {
        let angle_offset =
            rng.random_range(-std::f32::consts::FRAC_PI_3..std::f32::consts::FRAC_PI_3);
        let angle = base_angle + angle_offset;

        let speed = rng.random_range(400.0..1000.0);
        let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;

        let radius = rng.random_range(2.0..4.0);
        let spark_mesh = meshes.add(Circle::new(radius));

        let lifetime_duration = rng.random_range(0.12..0.30);

        commands.spawn((
            HitParticle { velocity },
            Lifetime(Timer::from_seconds(lifetime_duration, TimerMode::Once)),
            Mesh2d(spark_mesh),
            MeshMaterial2d(spark_mat.clone()),
            Transform::from_translation(impact_pos),
        ));
    }
}
