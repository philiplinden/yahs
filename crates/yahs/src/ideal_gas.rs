//! Ideal gas equations.
#![allow(dead_code)]

use std::ops::{Add, Div, Mul, Sub};

use avian3d::{
    math::{Scalar, PI},
    prelude::*,
};
use bevy::prelude::*;

use crate::{
    constants::{GAS_CONSTANT, STANDARD_G},
    core::SimState,
    geometry::sphere_volume,
    units::{DensityUnit, MassUnit, PressureUnit, TemperatureUnit, VolumeUnit},
};

const R: f32 = GAS_CONSTANT; // [J/K-mol] Ideal gas constant shortened for convenience

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<IdealGas>();
    app.register_type::<GasSpecies>();
    app.register_type::<MolarMass>();
}

/// Volume (m³) of an ideal gas from its temperature (K), pressure (Pa),
/// mass (kg) and molar mass (kg/mol).
pub fn ideal_gas_volume(
    temperature: TemperatureUnit,
    pressure: PressureUnit,
    mass: MassUnit,
    species: &GasSpecies,
) -> VolumeUnit {
    VolumeUnit(
        (mass.0 / species.molar_mass.kilograms_per_mole()) * R * temperature.kelvin()
            / pressure.pascals(),
    )
}

/// Density (kg/m³) of an ideal gas from its temperature (K), pressure (Pa),
/// and molar mass (kg/mol)
pub fn ideal_gas_density(
    temperature: TemperatureUnit,
    pressure: PressureUnit,
    species: &GasSpecies,
) -> DensityUnit {
    DensityUnit(
        species.molar_mass.kilograms_per_mole() * pressure.pascals() / (R * temperature.kelvin()),
    )
}

/// Molar mass (kg/mol) of a substance.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub struct MolarMass(pub Scalar);

impl MolarMass {
    pub fn kilograms_per_mole(&self) -> f32 {
        self.0
    }
}

impl Mul<Scalar> for MolarMass {
    type Output = MolarMass;

    fn mul(self, rhs: Scalar) -> Self::Output {
        MolarMass(self.0 * rhs)
    }
}

impl Div<Scalar> for MolarMass {
    type Output = MolarMass;

    fn div(self, rhs: Scalar) -> Self::Output {
        MolarMass(self.0 / rhs)
    }
}

/// Molecular species of a gas.
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
/// Add this component to a 
#[derive(Default, Debug, Clone, PartialEq, Reflect)]
pub struct IdealGas {
    pub species: GasSpecies,
    pub mass: MassUnit,
    pub temperature: TemperatureUnit,
    pub pressure: PressureUnit,
}

impl IdealGas {
    pub fn new(
        species: GasSpecies,
        temperature: TemperatureUnit,
        pressure: PressureUnit,
        mass: MassUnit,
    ) -> Self {
        IdealGas {
            species,
            temperature,
            pressure,
            mass,
        }
    }

    pub fn volume(&self) -> VolumeUnit {
        ideal_gas_volume(self.temperature, self.pressure, self.mass, &self.species)
    }

    pub fn density(&self) -> DensityUnit {
        ideal_gas_density(self.temperature, self.pressure, &self.species)
    }

    pub fn with_mass(self, mass: f32) -> Self {
        Self { mass: MassUnit::from_kilograms(mass), ..self }
    }
}
