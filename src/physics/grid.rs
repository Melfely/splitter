use bevy::prelude::*;

use crate::physics::definitions::*;

pub fn update_spatial_grid(
    mut grid: ResMut<SpatialGrid>,
    query: Query<(Entity, &GlobalTransform), With<Collider>>,
) {
    grid.cells.clear();

    for (entity, transform) in query.iter() {
        let pos = transform.translation().truncate();
        let cell_x = (pos.x / CELL_SIZE).floor() as i32;
        let cell_y = (pos.y / CELL_SIZE).floor() as i32;

        grid.cells
            .entry((cell_x, cell_y))
            .or_insert_with(Vec::new)
            .push(entity);
    }
}
