//! Properties, attributes and functions related to the balloon.

use avian3d::{math::PI, prelude::*};
use bevy::prelude::*;

use crate::{
    gas::IdealGas,
    thermodynamics::{sphere_radius_from_volume, Volume},
    core::SimulationUpdateOrder,
    forces::{Weight, Drag, Buoyancy},
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
#[require(IdealGas, RigidBody(|| RigidBody::Dynamic), Weight, Drag, Buoyancy)]
pub struct Balloon {
    pub material_properties: BalloonMaterial,
    pub shape: Sphere,
}

impl Default for Balloon {
    fn default() -> Self {
        Balloon {
            material_properties: BalloonMaterial::default(),
            shape: Sphere::default(),
        }
    }
}

impl Balloon {
    pub fn volume(&self) -> Volume {
        Volume(self.shape.volume())
    }
}

/// The balloon is the surface of a [`Primitive3d`] that can be stretched
/// radially [`GasSpecies`] based on the pressure of the gas it contains.
#[derive(Bundle)]
pub struct BalloonBundle {
    pub balloon: Balloon,
    pub gas: IdealGas,
}

impl Default for BalloonBundle {
    fn default() -> Self {
        let balloon = Balloon::default();
        let volume = balloon.volume();
        BalloonBundle {
            balloon: Balloon::default(),
            gas: IdealGas::default().with_volume(volume),
        }
    }
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
    pub thickness: f32,            // thickness of the material (m)
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

fn update_balloon_from_gas(mut query: Query<(&mut Balloon, &IdealGas)>) {
    for (mut balloon, gas) in query.iter_mut() {
        let new_radius = sphere_radius_from_volume(gas.volume().m3());
        balloon.shape.radius = new_radius;
    }
}
