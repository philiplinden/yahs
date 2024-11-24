//! Ideal gas equations.

use avian3d::prelude::*;
use bevy::prelude::*;

use super::{Pressure, Temperature, Volume, Density, MolarMass};
use super::properties::{AVOGADRO_CONSTANT, BOLTZMANN_CONSTANT};

pub const R: f32 = BOLTZMANN_CONSTANT * AVOGADRO_CONSTANT; // [J/K-mol] Ideal gas constant

pub struct IdealGasPlugin;

impl Plugin for IdealGasPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GasSpecies>();
        // app.add_systems(Update, (
        //     // update_ideal_gas_pressure_from_volume,
        //     // update_ideal_gas_density_from_volume,
        // ));
    }
}

#[derive(Bundle)]
pub struct IdealGasBundle {
    pub gas: IdealGas,
    pub species: GasSpecies,
}

/// Molecular species of a gas.
/// TODO: load species from a file
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
pub struct GasSpecies {
    pub name: String,
    pub abbreviation: String,
    pub molar_mass: MolarMass, // [kg/mol] molar mass a.k.a. molecular weight
}

impl GasSpecies {
    /// Dry air.
    pub fn air() -> Self {
        GasSpecies {
            name: "Air".to_string(),
            abbreviation: "AIR".to_string(),
            molar_mass: MolarMass(0.0289647),
        }
    }

    pub fn helium() -> Self {
        GasSpecies {
            name: "Helium".to_string(),
            abbreviation: "He".to_string(),
            molar_mass: MolarMass(0.0040026),
        }
    }
}

impl Default for GasSpecies {
    fn default() -> Self {
        GasSpecies::air()
    }
}

#[allow(dead_code)]
impl GasSpecies {
    pub fn new(name: String, abbreviation: String, molar_mass: MolarMass) -> Self {
        GasSpecies {
            name,
            abbreviation,
            molar_mass,
        }
    }
}

/// A finite amount of an ideal gas. Mass properties depend on the
/// [`GasSpecies`]. A gas will expand to fill its [`BoundingVolume`].
#[derive(Component, Debug, Clone, PartialEq)]
#[require(GasSpecies)]
pub struct IdealGas {
    pub temperature: Temperature,
    pub pressure: Pressure,
    pub mass: Mass,
}

/// Volume (m³) of an ideal gas from its temperature (K), pressure (Pa),
/// mass (kg) and molar mass (kg/mol).
pub fn ideal_gas_volume(
    temperature: Temperature,
    pressure: Pressure,
    mass: Mass,
    species: &GasSpecies,
) -> Volume {
    Volume(
        (mass.value() / species.molar_mass.kilograms_per_mole())
            * R * temperature.kelvin()
            / pressure.pascal(),
    )
}

/// Density (kg/m³) of an ideal gas from its temperature (K), pressure (Pa),
/// and molar mass (kg/mol)
pub fn ideal_gas_density(
    temperature: Temperature,
    pressure: Pressure,
    species: &GasSpecies,
) -> Density {
    Density(species.molar_mass.kilograms_per_mole() * pressure.pascal() / (R * temperature.kelvin()))
}

#[allow(dead_code)]
/// Gage pressure (Pa) of an ideal gas. This is the relative pressure compared
/// to the ambient pressure. Use `Atmosphere::pressure()` to get ambient
/// conditions.
pub fn gage_pressure(pressure: Pressure, ambient_pressure: Pressure) -> Pressure {
    pressure - ambient_pressure
}
