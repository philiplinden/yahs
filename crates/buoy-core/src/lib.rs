#![allow(unused_imports)]
pub mod atmosphere;
pub mod constants;
pub mod core;
pub mod forces;
pub mod format;
pub mod geometry;
pub mod grid;
pub mod ideal_gas;
pub mod scene;
pub mod time;

pub use uom as units;

pub mod prelude {
    pub use crate::{
        atmosphere::Atmosphere,
        core::{BuoyPlugin, SimState},
        forces::{drag, scale_gravity},
        grid::{Precision, RootGrid, GRID_CELL_EDGE_LENGTH_METERS},
        ideal_gas::{GasSpecies, IdealGas},
    };
    pub use uom::si::{
        f32::{Mass, MassDensity, MolarMass, Pressure, ThermodynamicTemperature, Volume},
        mass::kilogram,
        mass_density::kilogram_per_cubic_meter,
        molar_mass::kilogram_per_mole,
        pressure::pascal,
        thermodynamic_temperature::kelvin,
        volume::cubic_meter,
    };
}

pub use core::BuoyPlugin;
