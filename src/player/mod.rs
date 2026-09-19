use crate::splitter_core::laser::*;
use crate::splitter_core::turret::*;
use crate::splitter_core::weapon::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (player_movement, player_mouse_aiming, player_weapon_input),
        );
    }
}

pub fn player_mouse_aiming(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut q_turret_target: Query<&mut TurretTarget, With<PlayerTurret>>,
) {
    // Fixes E0599: iter().next() safely returns an Option, bypassing the missing get_single()
    let Some(window) = q_windows.iter().next() else {
        return;
    };
    let Some((camera, camera_transform)) = q_camera.iter().next() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
        for mut target in q_turret_target.iter_mut() {
            target.world_pos = Some(world_position);
        }
    }
}

// Marker components to route inputs to the correct barrel
#[derive(Component)]
pub struct MainWeapon;

#[derive(Component)]
pub struct CoaxialWeapon;

#[derive(Component)]
pub struct PlayerTurret;

#[derive(Component)]
pub struct Player {
    pub move_speed: f32,
    pub turn_speed: f32,
    pub blink_distance: f32,
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let hull_color = materials.add(Color::srgb(0.3, 0.3, 0.3));
    let turret_color = materials.add(Color::srgb(0.5, 0.5, 0.5));
    let weapon_color = materials.add(Color::srgb(0.1, 0.1, 0.1));

    commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(60.0, 80.0))),
            MeshMaterial2d(hull_color),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Player {
                move_speed: 150.0,
                turn_speed: 2.5,
                blink_distance: 100.0,
            },
        ))
        .with_children(|parent| {
            // Coaxial Weapon (Fixed to front right of the hull)
            parent.spawn((
                Mesh2d(meshes.add(Rectangle::new(10.0, 40.0))),
                MeshMaterial2d(weapon_color.clone()),
                Transform::from_xyz(20.0, 40.0, -0.1),
                CoaxialWeapon, // Marker for Right Click[cite: 2]
                WeaponTrigger::default(),
                Weapon {
                    kind: WeaponKind::Laser {
                        intensity: 15.0,
                        focus: 0.8,
                        current_heat: 0.0,
                        max_heat: 100.0,
                        heating_rate: 80.0, // Overheats in ~1.25s of continuous firing
                        cooling_rate: 40.0, // Fully cools in 2.5s from max heat
                        recharge_timer: Timer::from_seconds(2.0, TimerMode::Once),
                        is_recharging: false,
                        pattern: LaserPattern::Lightning {
                            length: PLAYER_LASER_LENGTH,
                            segment_length: 50.0,
                            jitter: 15.0,
                        },
                        color: Color::srgb(0.0, 0.8, 1.0),
                    },
                },
            ));

            // Main Turret (Center of hull) - Aiming components attached here
            parent
                .spawn((
                    Mesh2d(meshes.add(Rectangle::new(40.0, 40.0))),
                    MeshMaterial2d(turret_color),
                    Transform::from_xyz(0.0, 0.0, 0.1),
                    Turret { turn_speed: 10.0 },
                    TurretTarget::default(),
                    PlayerTurret,
                ))
                .with_children(|turret| {
                    // Main Turret Barrel
                    turret.spawn((
                        Mesh2d(meshes.add(Rectangle::new(12.0, 50.0))),
                        MeshMaterial2d(weapon_color),
                        Transform::from_xyz(0.0, 30.0, -0.1),
                        MainWeapon, // Marker for Left Click[cite: 2]
                        WeaponTrigger::default(),
                        Weapon {
                            kind: WeaponKind::Projectile {
                                mass: 30.0,
                                speed: 300.0,
                                can_pierce: false,
                                aoe_max_range: None,
                                current_ammo: 10,
                                max_ammo: 10,
                                fire_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                                reload_timer: Timer::from_seconds(3.0, TimerMode::Once),
                                mesh: meshes.add(Circle::new(8.0)),
                                material: materials.add(Color::srgb(0.9, 0.1, 0.1)),
                            },
                        },
                    ));
                });
        });
}

// --------------------------------------------------------
// NEW: Map mouse clicks to the generic trigger components
// --------------------------------------------------------
pub fn player_weapon_input(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut q_main: Query<&mut WeaponTrigger, (With<MainWeapon>, Without<CoaxialWeapon>)>,
    mut q_coax: Query<&mut WeaponTrigger, (With<CoaxialWeapon>, Without<MainWeapon>)>,
) {
    let is_left_clicking = mouse_input.pressed(MouseButton::Left);
    let is_right_clicking = mouse_input.pressed(MouseButton::Right);

    for mut trigger in q_main.iter_mut() {
        trigger.is_firing = is_left_clicking; // Routes to the rotating turret[cite: 2]
    }

    for mut trigger in q_coax.iter_mut() {
        trigger.is_firing = is_right_clicking; // Routes to the fixed hull weapon[cite: 2]
    }
}

fn player_movement(
    mut query: Query<(&mut Transform, &Player)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in query.iter_mut() {
        let mut forward_movement = 0.0;
        let mut rotation_movement = 0.0;

        // Forward/Backward (W/S)
        if keyboard_input.pressed(KeyCode::KeyW) {
            forward_movement += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            forward_movement -= 1.0;
        }

        // Steering (A/D)
        if keyboard_input.pressed(KeyCode::KeyA) {
            rotation_movement += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            rotation_movement -= 1.0;
        }

        // Fixed: delta_seconds() is now delta_secs()
        transform.rotate_z(rotation_movement * player.turn_speed * time.delta_secs());

        let up = transform.local_y();
        transform.translation += up * forward_movement * player.move_speed * time.delta_secs();

        // Blink (Q/E)
        if keyboard_input.just_pressed(KeyCode::KeyQ) {
            let left = -transform.local_x();
            transform.translation += left * player.blink_distance;
        }
        if keyboard_input.just_pressed(KeyCode::KeyE) {
            let right = transform.local_x();
            transform.translation += right * player.blink_distance;
        }
    }
}
