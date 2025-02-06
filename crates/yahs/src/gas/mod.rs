mod atmosphere;
mod ideal_gas;

use bevy::prelude::*;

// re-export
pub use atmosphere::Atmosphere;
pub use ideal_gas::{
    DebugGasSpecies, GasSpecies, IdealGas, ideal_gas_density, MolarMass,
};

pub(crate) use atmosphere::plugin as atmosphere_plugin;
pub(crate) use ideal_gas::plugin as ideal_gas_plugin;
