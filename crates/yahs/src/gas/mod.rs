mod atmosphere;
mod ideal_gas;

// re-export
pub use atmosphere::{AtmospherePlugin, Atmosphere};
pub use ideal_gas::{IdealGasPlugin, MolarMass, GasSpecies, IdealGas, IdealGasBundle, ideal_gas_density};
