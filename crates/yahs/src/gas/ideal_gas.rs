//! Ideal gas equations.
#![allow(dead_code)]

use std::ops::{Add, Div, Mul, Sub};

use avian3d::{
    prelude::*,
    math::{Scalar, PI},
};
use bevy::prelude::*;

use crate::{
    gas::Atmosphere,
    geometry::Volume,
    thermodynamics::{
        Density, Pressure, Temperature, AVOGADRO_CONSTANT, BOLTZMANN_CONSTANT,
    },
};

pub const R: f32 = BOLTZMANN_CONSTANT * AVOGADRO_CONSTANT; // [J/K-mol] Ideal gas constant

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

pub struct IdealGasPlugin;

impl Plugin for IdealGasPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GasSpecies>();
        app.register_type::<MolarMass>();
        app.add_systems(
            PreUpdate,
            (
                init_ideal_gas_density,
                update_ideal_gas_from_atmosphere,
                update_volume_from_pressure,
                update_volume_from_temperature,
            ).chain(),
        );
    }
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

#[derive(Component, Default, Debug, Clone, PartialEq, Reflect)]
#[require(Temperature, Pressure, Volume, Density, Mass, GasSpecies)]
pub struct IdealGas;

/// Properties of an ideal gas. For properties per unit mass, set the mass to 1.
#[derive(Bundle, Default, Debug, Clone, Reflect)]
pub struct IdealGasBundle {
    pub temperature: Temperature,
    pub pressure: Pressure,
    pub mass: Mass,
    pub species: GasSpecies,
}

impl IdealGasBundle {
    pub fn new(species: GasSpecies) -> Self {
        let temperature = Temperature::default();
        let pressure = Pressure::default();
        let mass = Mass(1.0);
        IdealGasBundle {
            temperature,
            pressure,
            species,
            mass,
        }
    }
}

fn init_ideal_gas_density(mut commands: Commands, mut query: Query<(Entity, &Pressure, &Temperature, &GasSpecies), Added<IdealGas>>) {
    for (entity, pressure, temperature, species) in query.iter_mut() {
        let density = ideal_gas_density(*temperature, *pressure, species);
        commands.entity(entity).insert(density);
    }
}

fn update_ideal_gas_from_atmosphere(
    mut query: Query<(&mut Pressure, &mut Temperature, &Position), (With<IdealGas>, Changed<Position>)>,
    atmosphere: Res<Atmosphere>,
) {
    for (mut pressure, mut temperature, position) in query.iter_mut() {
        *pressure = atmosphere.pressure(position.0);
        *temperature = atmosphere.temperature(position.0);
    }
}

fn update_volume_from_pressure(
    mut query: Query<(&mut Volume, &Pressure, &Temperature, &Mass, &GasSpecies), (With<IdealGas>, Changed<Pressure>)>,
) {
    for (mut volume, pressure, temperature, mass, species) in query.iter_mut() {
        volume.0 = ideal_gas_volume(*temperature, *pressure, *mass, species).m3();
    }
}

fn update_volume_from_temperature(
    mut query: Query<(&mut Volume, &Pressure, &Temperature, &Mass, &GasSpecies), (With<IdealGas>, Changed<Temperature>)>,
) {
    for (mut volume, pressure, temperature, mass, species) in query.iter_mut() {
        volume.0 = ideal_gas_volume(*temperature, *pressure, *mass, species).m3();
    }
}
