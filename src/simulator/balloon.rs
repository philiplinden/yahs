//! Properties, attributes and functions related to the balloon.

use avian3d::{math::PI, prelude::*};
use bevy::{prelude::*, math::primitives::Sphere};
#[cfg(feature = "config-files")]
use serde::{Deserialize, Serialize};

use super::{
    SimulatedBody,
    ideal_gas::{GasSpecies, IdealGasBundle},
    properties::*,
};

pub struct BalloonPlugin;

impl Plugin for BalloonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_balloon);

        // Register types for reflection
        app.register_type::<Balloon>();
        app.register_type::<BalloonMaterial>();
    }
}

#[derive(Bundle)]
pub struct BalloonBundle {
    pub balloon: Balloon,
    pub gas: IdealGasBundle,
}

#[derive(Debug, Clone, PartialEq, Reflect)]
#[cfg_attr(feature = "config-files", derive(Serialize, Deserialize))]
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

/// Balloon properties. The balloon always conforms to the surface of a
/// collider. It does not have its own rigid body.
#[derive(Component, Reflect)]
#[cfg_attr(feature = "config-files", derive(Serialize, Deserialize))]
pub struct Balloon {
    /// Balloon material type
    pub skin_material: BalloonMaterial,
    /// Thickness of balloon membrane in meters. For use in calculating stress.
    pub unstretched_thickness: f32,
    /// surface area of balloon without stretch (m²). For use in calculating stress.
    pub unstretched_area: f32,
}

impl Default for Balloon {
    fn default() -> Self {
        Balloon {
            skin_material: BalloonMaterial::default(),
            unstretched_thickness: 0.001,
            unstretched_area: 4.0 * PI,
        }
    }
}

fn spawn_balloon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 0.3;
    let shape = Sphere::new(radius);
    commands.spawn(
        Name::new("Balloon"),
        SimulatedBody,
        BalloonBundle {
            balloon: Balloon::default(),
            gas: IdealGasBundle::new(
                shape,
                GasSpecies::helium(),
                Temperature::STANDARD,
                Pressure::STANDARD,
            ),
            mesh: meshes.add(sphere),
        },
        RigidBody::Dynamic,
    );
}

// The balloon is stretched around the volume of the gas inside of it.
// This system updates the balloon's volume and radius based on the gas
// pressure.
// fn update_balloon_mesh(
//     asset_server: Res<AssetServer>,
//     mut query: Query<(&mut Mesh3d, &mut Shape, &Volume), With<Balloon>>,
// ) {
//     for (mut mesh, mut shape, volume) in query.iter_mut() {
//         shape.radius = sphere_radius_from_volume(*volume);
//     }
// }
