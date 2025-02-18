//! Ideal gas equations.
#![allow(dead_code)]

use std::ops::{Div, Mul};

use avian3d::math::Scalar;
use bevy::prelude::*;
use uom::si::{
    f32::{
        ThermodynamicTemperature, Pressure, Mass, Volume, 
        MassDensity, MolarMass
    },
    thermodynamic_temperature::kelvin,
    pressure::pascal,
    mass::kilogram,
    volume::cubic_meter,
    mass_density::kilogram_per_cubic_meter,
    molar_mass::kilogram_per_mole,
};

use crate::{
    constants::{GAS_CONSTANT, STANDARD_GRAVITY},
    core::SimState,
    geometry::sphere_volume,
};

pub(crate) fn plugin(app: &mut App) {
    // nothing yet
}

/// Volume (m³) of an ideal gas from its temperature (K), pressure (Pa),
/// mass (kg) and molar mass (kg/mol).
pub fn ideal_gas_volume(
    temperature: ThermodynamicTemperature,
    pressure: Pressure,
    mass: Mass,
    species: &GasSpecies,
) -> Volume {
    (mass / species.molar_mass) * *GAS_CONSTANT * temperature / pressure
}

/// Density (kg/m³) of an ideal gas from its temperature (K), pressure (Pa),
/// and molar mass (kg/mol)
pub fn ideal_gas_density(
    temperature: ThermodynamicTemperature,
    pressure: Pressure,
    species: &GasSpecies,
) -> MassDensity {
    species.molar_mass * pressure / (*GAS_CONSTANT * temperature)
}

/// Molecular species of a gas.
#[derive(Component, Debug, Clone, PartialEq)]
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
            molar_mass: MolarMass::new::<kilogram_per_mole>(0.0289647),
        }
    }

    pub fn helium() -> Self {
        GasSpecies {
            name: "Helium".to_string(),
            abbreviation: "He".to_string(),
            molar_mass: MolarMass::new::<kilogram_per_mole>(0.0040026),
        }
    }
}

impl Default for GasSpecies {
    fn default() -> Self {
        GasSpecies::helium()
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

/// Properties of an ideal gas per unit mass.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct IdealGas {
    pub species: GasSpecies,
    pub mass: Mass,
    pub temperature: ThermodynamicTemperature,
    pub pressure: Pressure,
}

impl IdealGas {
    pub fn new(
        species: GasSpecies,
        temperature: ThermodynamicTemperature,
        pressure: Pressure,
        mass: Mass,
    ) -> Self {
        IdealGas {
            species,
            temperature,
            pressure,
            mass,
        }
    }

    pub fn volume(&self) -> Volume {
        ideal_gas_volume(self.temperature, self.pressure, self.mass, &self.species)
    }

    pub fn density(&self) -> MassDensity {
        ideal_gas_density(self.temperature, self.pressure, &self.species)
    }

    pub fn with_mass(self, mass: f32) -> Self {
        Self {
            mass: Mass::new::<kilogram>(mass),
            ..self
        }
    }
}
