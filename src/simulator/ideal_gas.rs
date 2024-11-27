//! Ideal gas equations.
#![allow(dead_code)]
use avian3d::prelude::*;
use bevy::prelude::*;

use super::properties::{AVOGADRO_CONSTANT, BOLTZMANN_CONSTANT};
use super::{Atmosphere, Density, MolarMass, Pressure, SimulationUpdateOrder, Temperature, Volume};

pub const R: f32 = BOLTZMANN_CONSTANT * AVOGADRO_CONSTANT; // [J/K-mol] Ideal gas constant

pub struct IdealGasPlugin;

impl Plugin for IdealGasPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GasSpecies>();
        app.add_systems(
            Update,
            update_ideal_gas_from_atmosphere.in_set(SimulationUpdateOrder::IdealGas),
        );
    }
}

/// Molecular species of a gas.
#[derive(Debug, Clone, PartialEq, Reflect)]
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

/// Properties of an ideal gas. For properties per unit mass, set the mass to 1.
#[derive(Component, Debug, Clone, PartialEq)]
pub struct IdealGas {
    pub temperature: Temperature,
    pub pressure: Pressure,
    pub density: Density,
    pub mass: Mass,
    pub species: GasSpecies,
}

impl Default for IdealGas {
    fn default() -> Self {
        let species = GasSpecies::default();
        let temperature = Temperature::default();
        let pressure = Pressure::default();
        let density = ideal_gas_density(temperature, pressure, &species);
        let mass = Mass::new(1.0);
        IdealGas {
            temperature,
            pressure,
            density,
            species,
            mass,
        }
    }
}

impl IdealGas {
    pub fn new(species: GasSpecies) -> Self {
        let temperature = Temperature::default();
        let pressure = Pressure::default();
        let mass = Mass::new(1.0);
        let density = ideal_gas_density(temperature, pressure, &species);
        IdealGas {
            temperature,
            pressure,
            density,
            species,
            mass,
        }
    }

    pub fn with_temperature(mut self, temperature: Temperature) -> Self {
        self.temperature = temperature;
        self.update_density();
        self
    }

    pub fn with_pressure(mut self, pressure: Pressure) -> Self {
        self.pressure = pressure;
        self.update_density();
        self
    }

    pub fn with_mass(mut self, mass: Mass) -> Self {
        self.mass = mass;
        self
    }

    pub fn with_volume(mut self, volume: Volume) -> Self {
        self.mass = Mass::new(self.density.kg_per_m3() * volume.m3());
        self
    }

    fn update_density(&mut self) {
        self.density = ideal_gas_density(self.temperature, self.pressure, &self.species);
    }

    pub fn volume(&self) -> Volume {
        ideal_gas_volume(self.temperature, self.pressure, self.mass, &self.species)
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
        (mass.value() / species.molar_mass.kilograms_per_mole()) * R * temperature.kelvin()
            / pressure.pascals(),
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
        species.molar_mass.kilograms_per_mole() * pressure.pascals() / (R * temperature.kelvin()),
    )
}

#[allow(dead_code)]
/// Gage pressure (Pa) of an ideal gas. This is the relative pressure compared
/// to the ambient pressure. Use `Atmosphere::pressure()` to get ambient
/// conditions.
pub fn gage_pressure(pressure: Pressure, ambient_pressure: Pressure) -> Pressure {
    pressure - ambient_pressure
}

fn update_ideal_gas_from_atmosphere(
    mut query: Query<(&mut IdealGas, &Position)>,
    atmosphere: Res<Atmosphere>,
) {
    for (mut gas, position) in query.iter_mut() {
        gas.pressure = atmosphere.pressure(position.0);
        gas.temperature = atmosphere.temperature(position.0);
        gas.update_density();
    }
}
