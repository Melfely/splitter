use bevy::prelude::*; // References existing weapon structures[cite: 2]

/// Binds a UI element directly to a target weapon entity (turret, coaxial, etc.)
#[derive(Component)]
pub struct WeaponUiCard {
    pub target_weapon: Entity,
}

#[derive(Component)]
pub struct UiAmmoText;

#[derive(Component)]
pub struct UiHeatBarInner;

#[derive(Component)]
pub struct UiLaserStatusText;

#[derive(Component)]
pub struct UiBlinkBarQ;

#[derive(Component)]
pub struct UiBlinkBarE;

#[derive(Component)]
pub struct UiWeaponContainer;
