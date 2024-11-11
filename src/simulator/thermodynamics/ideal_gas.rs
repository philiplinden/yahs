//! Ideal gas equations.

#![allow(dead_code)]

use std::ops::{Div, Mul};

use avian3d::{math::Scalar, prelude::*};
use bevy::prelude::*;
use serde::Deserialize;

use super::{Density, Pressure, Temperature, Volume, R};

pub struct IdealGasPlugin;

impl Plugin for IdealGasPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<IdealGas>();
        app.register_type::<GasSpecies>();
        app.register_type::<MolarMass>();

        app.add_systems(Update, (
            update_ideal_gas_volume_from_pressure,
        ));
    }
}

/// Volume (m³) of an ideal gas from its temperature (K), pressure (Pa),
/// mass (kg) and molar mass (kg/mol).
pub fn volume(
    temperature: Temperature,
    pressure: Pressure,
    mass: Mass,
    molar_mass: MolarMass,
) -> f32 {
    (mass.0 / molar_mass.kilograms_per_mole()) * R * temperature.kelvin() / pressure.pascal()
    // [m³]
}

fn ideal_gas_volume(temperature: Temperature, pressure: Pressure, mass: Mass, species: &GasSpecies) -> Volume {
    Volume(volume(temperature, pressure, mass, species.molar_mass))
}

/// Density (kg/m³) of an ideal gas frorm its temperature (K), pressure (Pa),
/// and molar mass (kg/mol)
pub fn density(temperature: Temperature, pressure: Pressure, molar_mass: MolarMass) -> f32 {
    (molar_mass.0 * pressure.pascal()) / (R * temperature.kelvin()) // [kg/m³]
}

/// Molar mass (kg/mol) of a gas species
#[derive(Component, Debug, Deserialize, Clone, Copy, Reflect)]
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

#[derive(Component, Debug, Deserialize, Clone, Reflect)]
pub struct GasSpecies {
    pub name: String,
    pub abbreviation: String,
    pub molar_mass: MolarMass, // [kg/mol] molar mass a.k.a. molecular weight
}

impl Default for GasSpecies {
    fn default() -> Self {
        GasSpecies::new(
            "Air".to_string(),
            "AIR".to_string(),
            MolarMass(0.0289647),
        )
    }
}

impl GasSpecies {
    pub fn new(name: String, abbreviation: String, molar_mass: MolarMass) -> Self {
        GasSpecies {
            name,
            abbreviation,
            molar_mass,
        }
    }
}

/// A finite amount of a particular ideal gas
#[derive(Component, Debug, Reflect)]
pub struct IdealGas {
    pub species: GasSpecies,
    pub mass: Mass,
    pub temperature: Temperature,
    pub pressure: Pressure,
    pub volume: Volume,
}

impl IdealGas {
    pub fn from_mass(
        species: GasSpecies,
        mass: Mass,
        temperature: Temperature,
        pressure: Pressure,
    ) -> Self {
        // Create a new gas volume as a finite amount of mass (kg) of a
        // particular species of gas. Gas is initialized at standard
        // temperature and pressure.
        IdealGas {
            species: species.clone(),
            mass,
            volume: ideal_gas_volume(
                temperature,
                pressure,
                mass,
                &species,
            ),
            temperature,
            pressure,
        }
    }


    pub fn from_volume(
        species: GasSpecies,
        volume: Volume,
        temperature: Temperature,
        pressure: Pressure,
    ) -> Self {
        // Create a new gas volume as a finite amount of mass (kg) of a
        // particular species of gas. Gas is initialized at standard
        // temperature and pressure.
        IdealGas {
            species: species.clone(),
            mass: Mass(volume.0 * density(temperature, pressure, species.molar_mass)),
            volume,
            temperature,
            pressure,
        }
    }

    /// Ideal gas temperature (K)
    pub fn temperature(&self) -> f32 {
        self.temperature.kelvin()
    }

    /// Pressure (Pa)
    pub fn pressure(&self) -> f32 {
        self.pressure.pascal()
    }

    /// Ideal gas density (kg/m³)
    pub fn density(&self) -> f32 {
        Density::from_gas(self.temperature, self.pressure, self.species.molar_mass).kg_per_m3()
    }

    /// Ideal gas volume (m³)
    pub fn volume(&self) -> f32 {
        self.volume.cubic_meters()
    }
}

impl Default for IdealGas {
    fn default() -> Self {
        IdealGas {
            species: GasSpecies::default(),
            mass: Mass::ZERO,
            volume: Volume::ZERO,
            temperature: Temperature::STANDARD,
            pressure: Pressure::STANDARD,
        }
    }
}

fn update_ideal_gas_volume_from_pressure(mut query: Query<&mut IdealGas>) {
    for mut gas in query.iter_mut() {
        gas.volume = ideal_gas_volume(gas.temperature, gas.pressure, gas.mass, &gas.species);
    }
}
