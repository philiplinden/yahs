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
        app.register_type::<GasSpecies>();
        app.register_type::<MolarMass>();

        app.add_systems(Update, update_ideal_gas_volume_from_pressure);
    }
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

impl GasSpecies {
    /// Dry air.
    pub fn air() -> Self {
        GasSpecies {
            name: "Air".to_string(),
            abbreviation: "AIR".to_string(),
            molar_mass: MolarMass(0.0289647),
        }
    }
}

impl Default for GasSpecies {
    fn default() -> Self {
        GasSpecies::air()
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

/// Volume (m³) of an ideal gas from its temperature (K), pressure (Pa),
/// mass (kg) and molar mass (kg/mol).
pub fn ideal_gas_volume(
    temperature: Temperature,
    pressure: Pressure,
    mass: Mass,
    species: &GasSpecies,
) -> Volume {
    Volume(
        (mass.0 / species.molar_mass.kilograms_per_mole()) * R * temperature.kelvin()
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
    Density(
        (species.molar_mass.kilograms_per_mole() * pressure.pascal()) / (R * temperature.kelvin()),
    )
}

/// A finite amount of a particular ideal gas
#[derive(Component, Debug)]
pub struct IdealGas;

#[derive(Bundle, Debug)]
pub struct IdealGasBundle {
    pub collider: Collider,
    pub species: GasSpecies,
    pub temperature: Temperature,
    pub pressure: Pressure,
    pub volume: Volume,
}

impl IdealGasBundle {
    pub fn new(
        collider: Collider,
        species: GasSpecies,
        temperature: Temperature,
        pressure: Pressure,
    ) -> Self {
        let density = ideal_gas_density(temperature, pressure, &species);
        let mass = collider.mass_properties(density.kg_per_m3()).mass;
        Self {
            collider,
            species: species.clone(),
            temperature,
            pressure,
            volume: ideal_gas_volume(temperature, pressure, mass, &species),
        }
    }
}

impl Default for IdealGasBundle {
    fn default() -> Self {
        IdealGasBundle::new(
            Collider::sphere(1.0),
            GasSpecies::default(),
            Temperature::STANDARD,
            Pressure::STANDARD,
        )
    }
}

fn update_ideal_gas_volume_from_pressure(
    mut query: Query<(&mut Volume, &Temperature, &Pressure, &Mass, &GasSpecies), With<IdealGas>>,
) {
    for (mut volume, temperature, pressure, mass, species) in query.iter_mut() {
        *volume = ideal_gas_volume(*temperature, *pressure, *mass, &species);
    }
}
