use crate::physics::collisions::check_collisions;
use crate::physics::definitions::{CollisionMessage, SpatialGrid};
use crate::physics::grid::update_spatial_grid;

use bevy::prelude::*;

pub mod collisions;
pub mod definitions;
pub mod grid;
pub mod laser;

pub struct SplitterPhysicsPlugin;

impl Plugin for SplitterPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpatialGrid>()
            .add_message::<CollisionMessage>() // Replaces add_event
            .add_systems(
                Update,
                (
                    update_spatial_grid,
                    check_collisions.after(update_spatial_grid),
                ),
            );
    }
}
