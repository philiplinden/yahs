#![allow(unused_imports)]
pub mod atmosphere;
pub mod constants;
pub mod core;
pub mod forces;
pub mod geometry;
pub mod ideal_gas;
pub mod space;
pub mod time;
pub mod format;

pub mod prelude {
    pub use crate::{
        atmosphere::Atmosphere,
        core::{SimState, BuoyPlugin},
        forces::{drag, scale_gravity},
        space::{GridPrecision, GRID_CELL_EDGE_LENGTH_METERS},
        time::{StepPhysicsEvent, TimeScaleOptions},
        ideal_gas::{GasSpecies, IdealGas},
    };
}

pub use core::BuoyPlugin;
