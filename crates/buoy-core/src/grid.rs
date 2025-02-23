use bevy::prelude::*;
use big_space::prelude::*;
use uom::si::{f32::*, length::meter, volume::cubic_meter};

/// The size of the grid cells in space.
pub const GRID_CELL_EDGE_LENGTH_METERS: f32 = 10.0;
pub const GRID_SWITCHING_THRESHOLD_METERS: f32 = 0.5;

/// The precision of the grid.
#[cfg(all(feature = "i32", not(any(feature = "i64", feature = "i128"))))]
pub type Precision = i32;
#[cfg(all(feature = "i64", not(any(feature = "i32", feature = "i128"))))]
pub type Precision = i64;
#[cfg(all(feature = "i128", not(any(feature = "i32", feature = "i64"))))]
pub type Precision = i128;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(BigSpacePlugin::<Precision>::default());
    app.add_systems(Startup, setup_worldspace);
}

/// Sets up the worldspace root node of the grid hierarchy. This is the root
/// node of the world space hierarchy.
///
/// Use child grids to specify large environment volumes, like atmosphere or
/// ocean. New spatial entities should be spawned with their own local grids.
/// Objects can move across environment boundaries so long as they are related
/// along the same branch of the hierarchy.
///
/// The switching threshold (set to 0.0 here) determines when grid cells switch.
/// A non-zero threshold creates a "buffer zone" around cell boundaries to prevent
/// rapid switching when objects oscillate near the edge. For example:
///
/// ```text
/// Cell 1          |          Cell 2
///                 |
///         <--🚀-->|   (threshold = 0.0)
///                 |
///     [===buffer zone===]   (threshold > 0.0)
/// ```
///
/// With threshold = 0.0, the object triggers an immediate cell switch when crossing
/// the boundary. A positive threshold allows some movement past the boundary before
/// switching, preventing jitter for objects that frequently cross cell edges.
fn setup_worldspace(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn the root node of the grid hierarchy. This is the grid that
    // contains the entire world.
    let world_grid = Grid::<Precision>::new(
        GRID_CELL_EDGE_LENGTH_METERS,
        GRID_SWITCHING_THRESHOLD_METERS,
    );
    commands.spawn_big_space(world_grid, |root_grid| {
        // A dummy entity to represent the starting spot of the world.
        root_grid.insert(RootGrid);
        root_grid.spawn_spatial((
            Name::new("Starting Spot"),
            StartingSpot,
            FloatingOrigin,
            Transform::default(),
            GlobalTransform::default(),
        ));
    });
}

/// A marker component for the starting spot of the world.
/// This is used to spawn entities at startup. The FloatingOrigin shares this
/// location until it is attached to a spatial entity or camera.
#[derive(Component)]
pub struct StartingSpot;

/// A marker component for the root grid.
#[derive(Component)]
pub struct RootGrid;

/// A marker component for a grid that contains a fluid volume.
#[derive(Component)]
pub struct FluidVolumeGrid;
