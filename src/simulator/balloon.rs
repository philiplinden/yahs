//! Properties, attributes and functions related to the balloon.

use avian3d::{math::PI, prelude::Position};
use bevy::prelude::*;

use super::{
    SimulatedBody,
    SimulationUpdateOrder,
    ideal_gas::IdealGas,
    properties::sphere_radius_from_volume,
};

pub struct BalloonPlugin;

impl Plugin for BalloonPlugin {
    fn build(&self, app: &mut App) {
        // Register types for reflection
        app.register_type::<Balloon>();
        app.register_type::<BalloonMaterial>();

        app.add_systems(
            Update,
            update_balloon_from_gas.in_set(SimulationUpdateOrder::MeshVolumes),
        );
    }
}

#[derive(Component, Debug, Clone, PartialEq, Reflect)]
pub struct Balloon;

/// The balloon is the surface of a [`Primitive3d`] that can be stretched
/// radially [`GasSpecies`] based on the pressure of the gas it contains.
#[derive(Bundle)]
pub struct BalloonBundle {
    pub material_properties: BalloonMaterial,
    pub gas: IdealGas,
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
}

#[derive(Component, Debug, Clone, PartialEq, Reflect)]
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
    pub thickness: f32,             // thickness of the material (m)
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
            thickness: 0.0001,
        }
    }
}

fn update_balloon_from_gas(mut query: Query<(&mut Mesh3d, &IdealGas)>) {
    for (mut mesh, gas) in query.iter_mut() {
        // let new_radius = sphere_radius_from_volume(gas.volume().m3());
        // mesh.0.scale = Vec3::new(new_radius, new_radius, new_radius);
    }
}
