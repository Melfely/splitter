use crate::camera::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::splitter_core::weapon::Weapon;
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerPhysics {
    pub velocity: Vec2,
    pub angular_velocity: f32,
    pub acceleration: f32,
    pub max_speed: f32,
    pub turn_acceleration: f32,
    pub max_turn_speed: f32,
    pub forward_drag: f32,
    pub lateral_drag: f32,
    pub angular_drag: f32,
    pub blink_impulse: f32,
    pub blink_distance: f32,
}

impl Default for PlayerPhysics {
    fn default() -> Self {
        Self {
            velocity: Vec2::ZERO,
            angular_velocity: 0.0,
            acceleration: 200.0,
            max_speed: 120.0,
            turn_acceleration: 5.0,
            max_turn_speed: 2.25,
            forward_drag: 1.0,    // Low friction: coasting / momentum
            lateral_drag: 8.0,    // High friction: heavy tread resistance
            angular_drag: 3.0,    // Rapid rotational damping
            blink_impulse: 800.0, // Slide momentum added post-teleport
            blink_distance: 200.0,
        }
    }
}

#[derive(Component)]
pub struct PlayerBlink {
    pub q_cooldown: Timer,
    pub e_cooldown: Timer,
}

impl Default for PlayerBlink {
    fn default() -> Self {
        Self {
            q_cooldown: Timer::from_seconds(1.5, TimerMode::Once),
            e_cooldown: Timer::from_seconds(1.5, TimerMode::Once),
        }
    }
}

/// Marker component for identifying mount locations on player/entities.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MountLocation {
    Turret,
    Coaxial,
}

/// Defines the mount point type and relative placement.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MountPoint {
    /// Rotating turret mount. Standard weapon stats.
    Turret { offset: Vec3 },
    /// Fixed hull mount. Grants +10% projectile speed boost.
    Coaxial { offset: Vec3 },
    /// Dual rotating turret mount. 50% projectile mass penalty.
    DualTurret { offset: Vec3, spacing: f32 },
    /// Dual fixed hull mount. 50% projectile mass penalty and +10% speed boost.
    DualCoaxial { offset: Vec3, spacing: f32 },
}

/// Fully self-contained weapon mounting hook spec assigned from external configuration.
#[derive(Clone)]
pub struct MountedWeapon {
    pub weapon: Weapon,
    pub mount_point: MountPoint,
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

#[derive(Resource, Default, Clone)]
pub struct PlayerLoadout {
    pub coaxial_mounts: Vec<MountedWeapon>,
    pub turret_mounts: Vec<MountedWeapon>,
}

// Marker components to route inputs to the correct barrel
#[derive(Component)]
pub struct MainWeapon;

#[derive(Component)]
pub struct CoaxialWeapon;

#[derive(Component)]
pub struct PlayerTurret;

#[derive(Component)]
pub struct Player {}

pub const ARENA_HALF_WIDTH: f32 = ARENA_WIDTH / 2.0;
pub const ARENA_HALF_HEIGHT: f32 = ARENA_HEIGHT / 2.0;
pub const OUT_OF_BOUNDS_DAMAGE_PER_SEC: f32 = 10.0;
