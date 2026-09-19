use bevy::prelude::*;

#[derive(Clone, Debug)]
pub enum LaserPattern {
    /// Standard straight beam raycast.
    Straight { length: f32 },
    /// Fractured bolt with jagged offsets. Segment count scales dynamically with length.
    Lightning {
        length: f32,
        segment_length: f32, // Desired length of each zig-zag segment
        jitter: f32,
    },
    /// Sine wave style energy beam. Step resolution scales automatically over distance.
    PulseWave {
        length: f32,
        frequency: f32,
        amplitude: f32,
    },
}

impl LaserPattern {
    /// Generates world-space 2D points along the beam path.
    pub fn generate_points(&self, origin: Vec2, direction: Vec2, time_secs: f32) -> Vec<Vec2> {
        let perpendicular = Vec2::new(-direction.y, direction.x);

        match self {
            LaserPattern::Straight { length } => {
                vec![origin, origin + direction * *length]
            }
            LaserPattern::Lightning {
                length,
                segment_length,
                jitter,
            } => {
                // Calculate total segments dynamically so density stays high across off-screen distances
                let num_segments = (*length / *segment_length).max(2.0) as usize;
                let mut points = Vec::with_capacity(num_segments + 1);
                points.push(origin);

                let step = *length / num_segments as f32;
                for i in 1..num_segments {
                    let progress = i as f32 * step;
                    let seed = (i as f32 * 12.9898 + time_secs * 50.0).sin();
                    let offset = seed * *jitter;

                    points.push(origin + (direction * progress) + (perpendicular * offset));
                }

                points.push(origin + direction * *length);
                points
            }
            LaserPattern::PulseWave {
                length,
                frequency,
                amplitude,
            } => {
                // Calculate step density to keep wave curves smooth all the way off-screen
                let step_size = 15.0; // Point every 15 world units
                let num_segments = (*length / step_size).max(10.0) as usize;
                let mut points = Vec::with_capacity(num_segments + 1);
                let step = *length / num_segments as f32;

                for i in 0..=num_segments {
                    let progress = i as f32 * step;
                    let wave = ((progress * frequency) + (time_secs * 10.0)).sin();
                    let offset = wave * *amplitude;

                    points.push(origin + (direction * progress) + (perpendicular * offset));
                }
                points
            }
        }
    }
}
