use bevy::prelude::*;
use big_space::prelude::*;

use crate::units::{DistanceUnit, VolumeUnit};

/// The size of the grid cells in space.
const GRID_CELL_EDGE_LENGTH_METERS: f32 = 10.0;
type GridPrecision = i16;
type WorldCell = GridCell<GridPrecision>;

/// The edge length of one grid cell.
pub fn get_grid_cell_edge_length() -> DistanceUnit {
    DistanceUnit::from_meters(GRID_CELL_EDGE_LENGTH_METERS)
}

/// The volume of one grid cell.
pub fn get_grid_cell_volume() -> VolumeUnit {
    let grid_cell_edge_length = get_grid_cell_edge_length();
    VolumeUnit::from_cubic_meters(grid_cell_edge_length.0 * grid_cell_edge_length.0 * grid_cell_edge_length.0)
}

/// The edge length of the world's coordinate frame along one axis.
pub fn get_grid_world_edge_length() -> DistanceUnit {
    let num_grid_cells = usize::pow(GridPrecision::BITS as usize, 2) as f32;
    get_grid_cell_edge_length() * num_grid_cells
}

/// The volume of the world's coordinate frame.
pub fn get_grid_world_volume() -> VolumeUnit {
    let world_edge_length = get_grid_world_edge_length();
    VolumeUnit::from_cubic_meters(world_edge_length.0 * world_edge_length.0 * world_edge_length.0)
}

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(BigSpacePlugin::<GridPrecision>::default());
}
