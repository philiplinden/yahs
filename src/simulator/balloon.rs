//! Properties, attributes and functions related to the balloon.
#![allow(dead_code)]

use serde::Deserialize;
use std::fmt;
use bevy::prelude::*;

use crate::simulator::gas::GasVolume;

pub struct BalloonPlugin;

impl Plugin for BalloonPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_systems(Update, step);
    }
}

#[derive(Component)]
pub struct BalloonBundle {
    pub balloon: Balloon,
    pub gas: GasVolume,
    pub transform: TransformBundle,
}

#[derive(Component)]
pub struct Balloon {
    /// whether or not it has burst
    pub intact: bool,
    /// Balloon material type
    pub skin_material: BalloonMaterial,
    /// Thickness of balloon membrane in meters
    pub unstretched_thickness: f32,
    /// radius of balloon without stretch (m)
    pub unstretched_radius: f32,
    /// shape of the balloon
    pub mesh: Mesh,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Reflect)]
pub struct BalloonMaterial {
    pub name: String,
    pub max_temperature: f32, // temperature (K) where the given material fails
    pub density: f32,         // density (kg/m³)
    pub emissivity: f32,      // how much thermal radiation is emitted
    pub absorptivity: f32,    // how much thermal radiation is absorbed
    pub thermal_conductivity: f32, // thermal conductivity (W/mK) of the material at room temperature
    pub specific_heat: f32,        // J/kgK
    pub poissons_ratio: f32,       // ratio of change in width for a given change in length
    pub elasticity: f32,           // Youngs Modulus aka Modulus of Elasticity (Pa)
    pub max_strain: f32,           // elongation at failure (decimal, unitless) 1 = original size
    pub max_stress: f32,           // tangential stress at failure (Pa)
}

impl fmt::Display for BalloonMaterial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for BalloonMaterial {
    fn default() -> Self {
        BalloonMaterial {
            name: "Latex".to_string(),
            max_temperature: 373.0,     // Example value in Kelvin
            density: 920.0,             // Example density in kg/m³
            emissivity: 0.9,            // Example emissivity
            absorptivity: 0.9,          // Example absorptivity
            thermal_conductivity: 0.13, // Example thermal conductivity in W/mK
            specific_heat: 2000.0,      // Example specific heat in J/kgK
            poissons_ratio: 0.5,        // Example Poisson's ratio
            elasticity: 0.01e9,         // Example Young's Modulus in Pa
            max_strain: 0.8,            // Example max strain (unitless)
            max_stress: 0.5e6,          // Example max stress in Pa
        }
    }
}
