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
    debug,
    forces::{BuoyancyForce, Forces, WeightForce},
    gas::Atmosphere,
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

    pub fn debug() -> Self {
        let debug_species = DebugGasSpecies::default();
        GasSpecies {
            name: debug_species.name,
            abbreviation: debug_species.abbreviation,
            molar_mass: debug_species.molar_mass,
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

/// Imaginary gas species for debugging.
/// This gas species can be initialized to result in desired properties based
/// on a given set of parameters. By default it is initialized to the molar
/// mass of an imaginary gas that weighs 1 N for 1 kg at 1 atm and 20°C.
pub struct DebugGasSpecies {
    pub name: String,
    pub abbreviation: String,
    pub molar_mass: MolarMass,
}

impl DebugGasSpecies {
    /// Debug gas species.
    pub fn new(mass: f32, radius: f32, pressure: f32, temperature: f32) -> Self {
        let debug_pressure = PressureUnit::from_atmospheres(pressure).pascals(); // [Pa]
        let debug_temperature = TemperatureUnit::from_celsius(temperature).kelvin(); // [K]
        let debug_volume = VolumeUnit(sphere_volume(radius)).m3(); // [m³]

        let debug_moles = debug_pressure * debug_volume / (R * debug_temperature); // [mol]
        let debug_molar_mass = mass / debug_moles; // [kg/mol]

        DebugGasSpecies {
            name: "Debug".to_string(),
            abbreviation: "DBG".to_string(),
            molar_mass: MolarMass(debug_molar_mass),
        }
    }

    /// Gas species with a given weight and radius at STP.
    pub fn debug_stp_with_weight(weight: f32, radius: f32) -> Self {
        let debug_weight = weight; // [N]
        let debug_radius = radius; // [m]
        let debug_pressure = Atmosphere::standard_pressure().pascals(); // [Pa]
        let debug_temperature = Atmosphere::standard_temperature().kelvin(); // [K]
        let debug_volume = VolumeUnit(sphere_volume(debug_radius)).m3(); // [m³]
        let debug_mass = debug_weight / STANDARD_G; // [kg]
        let debug_moles = debug_pressure * debug_volume / (R * debug_temperature); // [mol]

        let debug_molar_mass = debug_mass / debug_moles; // [kg/mol]
        DebugGasSpecies {
            name: "Debug".to_string(),
            abbreviation: "DBG".to_string(),
            molar_mass: MolarMass(debug_molar_mass),
        }
    }

    /// Gas species with a given buoyancy and radius at STP.
    pub fn debug_stp_with_buoyancy(buoyancy: f32, radius: f32) -> Self {
        let debug_radius = radius; // [m]
        let debug_volume = VolumeUnit(sphere_volume(debug_radius)).m3(); // [m³]
        let air_density_at_stp = Atmosphere::standard_density().kg_per_m3(); // [kg/m³]
        let air_weight_at_stp = debug_volume * air_density_at_stp * STANDARD_G; // [N]

        let debug_weight = buoyancy * air_weight_at_stp; // [N]
        let debug_pressure = Atmosphere::standard_pressure().pascals(); // [Pa]
        let debug_temperature = Atmosphere::standard_temperature().kelvin(); // [K]
        let debug_mass = debug_weight / STANDARD_G; // [kg]
        let debug_moles = debug_pressure * debug_volume / (R * debug_temperature); // [mol]
        let debug_molar_mass = debug_mass / debug_moles; // [kg/mol]
        DebugGasSpecies {
            name: "Debug".to_string(),
            abbreviation: "DBG".to_string(),
            molar_mass: MolarMass(debug_molar_mass),
        }
    }
}

impl Default for DebugGasSpecies {
    /// The molar mass of an imaginary gas that weighs 1 N for 1 kg at 1 atm
    /// and 20°C.
    ///
    /// mass [kg] = 1 N / 9.80665 m/s^2
    /// volume [m³] = 4/3 * π * 1 m^3
    /// n [mol] = PV/RT
    /// M [kg/mol] = mass / n
    ///
    /// This is surprisingly close to the molar mass of neon (20.1797 g/mol).
    fn default() -> Self {
        let debug_mass = 1.0 / STANDARD_G; // [kg]
        DebugGasSpecies::new(debug_mass, 1.0, 1.0, 20.0)
    }
}

impl Into<GasSpecies> for DebugGasSpecies {
    fn into(self) -> GasSpecies {
        GasSpecies {
            name: self.name,
            abbreviation: self.abbreviation,
            molar_mass: self.molar_mass,
        }
    }
}

/// Properties of an ideal gas per unit mass.
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
