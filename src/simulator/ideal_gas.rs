//! Ideal gas equations.
#![allow(dead_code)]

use avian3d::prelude::*;
use bevy::prelude::*;
use serde::Deserialize;

use crate::simulator::properties::{Mass as SimMass, *};

pub const R: f32 = BOLTZMANN_CONSTANT * AVOGADRO_CONSTANT; // [J/K-mol] Ideal gas constant

pub struct IdealGasPlugin;

impl Plugin for IdealGasPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GasSpecies>();
        app.add_systems(Update, (
            update_ideal_gas_volume_from_pressure,
            update_ideal_gas_density_from_volume,
        ));
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
    mass: SimMass,
    species: &GasSpecies,
) -> Volume {
    Volume(
        (mass.kilograms() / species.molar_mass.kilograms_per_mole()) * R * temperature.kelvin()
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

/// Gage pressure (Pa) of an ideal gas. This is the relative pressure compared
/// to the ambient pressure. Use `Atmosphere::pressure()` to get ambient
/// conditions.
pub fn gage_pressure(pressure: Pressure, ambient_pressure: Pressure) -> Pressure {
    pressure - ambient_pressure
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
    pub mass: SimMass,
}

impl IdealGasBundle {
    pub fn new(
        collider: Collider,
        species: GasSpecies,
        temperature: Temperature,
        pressure: Pressure,
    ) -> Self {
        let density = ideal_gas_density(temperature, pressure, &species);
        let mass_props = collider.mass_properties(density.kg_per_m3());
        let mass = SimMass::from_mass_properties(mass_props);
        Self {
            collider,
            species: species.clone(),
            temperature,
            pressure,
            volume: ideal_gas_volume(temperature, pressure, mass, &species),
            mass,
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
    mut query: Query<(&mut Volume, &Temperature, &Pressure, &SimMass, &GasSpecies), With<IdealGas>>,
) {
    for (mut volume, temperature, pressure, mass, species) in query.iter_mut() {
        *volume = ideal_gas_volume(*temperature, *pressure, *mass, &species);
    }
}

fn update_ideal_gas_density_from_volume(
    mut query: Query<(&mut Density, &Volume, &SimMass), With<IdealGas>>,
) {
    for (mut density, volume, mass) in query.iter_mut() {
        *density = *mass / *volume;
    }
}
