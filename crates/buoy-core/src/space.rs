use bevy::prelude::*;
use big_space::prelude::*;
use uom::si::{f32::*, length::meter, volume::cubic_meter};

/// The size of the grid cells in space.
pub const GRID_CELL_EDGE_LENGTH_METERS: f32 = 10.0;
pub type GridPrecision = i64;
pub type WorldGrid = Grid<GridPrecision>;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(BigSpacePlugin::<GridPrecision>::default());
    app.add_systems(Startup, spawn_worldspace);
}

fn spawn_worldspace(mut commands: Commands) {
    commands.spawn_big_space_default::<GridPrecision>(|root| {
        root.with_grid_default(|worldspace| {
            worldspace.spawn_spatial((
                FloatingOrigin,
            ));
        });
    });
}

/// The edge length of one grid cell.
pub fn get_grid_cell_edge_length() -> Length {
    Length::new::<meter>(GRID_CELL_EDGE_LENGTH_METERS)
}

/// The volume of one grid cell.
pub fn get_grid_cell_volume() -> Volume {
    let grid_cell_edge_length = get_grid_cell_edge_length();
    grid_cell_edge_length * grid_cell_edge_length * grid_cell_edge_length
}

/// The edge length of the world's coordinate frame along one axis.
pub fn get_grid_world_edge_length() -> Length {
    let num_grid_cells = usize::pow(GridPrecision::BITS as usize, 2) as f32;
    get_grid_cell_edge_length() * num_grid_cells
}

/// The volume of the world's coordinate frame.
pub fn get_grid_world_volume() -> Volume {
    let world_edge_length = get_grid_world_edge_length();
    world_edge_length * world_edge_length * world_edge_length
}
