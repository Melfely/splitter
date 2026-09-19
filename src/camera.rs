use bevy::camera::{ScalingMode, Viewport};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

// Constant dimensions of your fixed play field in world units
pub const ARENA_WIDTH: f32 = 3840.0;
pub const ARENA_HEIGHT: f32 = 2160.0;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            // Lock orthographic scale strictly to play field dimensions
            scaling_mode: ScalingMode::Fixed {
                width: ARENA_WIDTH,
                height: ARENA_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

/// Dynamically adjusts camera viewport to enforce aspect ratio letterboxing
pub fn fit_camera_viewport(
    q_window: Query<&Window, (With<PrimaryWindow>, Changed<Window>)>,
    mut q_camera: Query<&mut Camera, With<Camera2d>>,
) {
    let Some(window) = q_window.iter().next() else {
        return;
    };
    let Some(mut camera) = q_camera.iter_mut().next() else {
        return;
    };

    let win_width = window.resolution.physical_width();
    let win_height = window.resolution.physical_height();

    if win_width == 0 || win_height == 0 {
        return;
    }

    let target_aspect = ARENA_WIDTH / ARENA_HEIGHT;
    let window_aspect = win_width as f32 / win_height as f32;

    // Calculate pillarbox (left/right bars) or letterbox (top/bottom bars)
    let (viewport_width, viewport_height) = if window_aspect > target_aspect {
        let height = win_height;
        let width = (height as f32 * target_aspect).round() as u32;
        (width, height)
    } else {
        let width = win_width;
        let height = (width as f32 / target_aspect).round() as u32;
        (width, height)
    };

    let x = (win_width - viewport_width) / 2;
    let y = (win_height - viewport_height) / 2;

    camera.viewport = Some(Viewport {
        physical_position: UVec2::new(x, y),
        physical_size: UVec2::new(viewport_width, viewport_height),
        ..default()
    });
}
