//! Properties, attributes and functions related to the balloon.

use avian3d::prelude::*;
use bevy::prelude::*;
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
    pub pbr: PbrBundle,
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
            unstretched_area: 4.0 * std::f32::consts::PI,
        }
    }
}
fn spawn_balloon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 0.3;
    commands.spawn((
        Name::new("BalloonBundle"),
        SimulatedBody,
        BalloonBundle {
            balloon: Balloon::default(),
            gas: IdealGasBundle::new(
                Collider::sphere(radius),
                GasSpecies::helium(),
                Temperature::STANDARD,
                Pressure::STANDARD,
            ),
            pbr: PbrBundle {
                mesh: meshes.add(Sphere::new(radius)),
                material: materials.add(Color::srgb_u8(124, 144, 255)),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
        },
        RigidBody::Dynamic,
    ));
}

// impl Balloon {

//     pub fn gage_pressure(&self, external_pressure: f32) -> f32 {
//         self.lift_gas.pressure() - external_pressure
//     }

//     fn set_stress(&mut self, external_pressure: f32) {
//         // hoop stress (Pa) of thin-walled hollow sphere from internal pressure
//         // https://en.wikipedia.org/wiki/Pressure_vessel#Stress_in_thin-walled_pressure_vessels
//         // https://pkel015.connect.amazon.auckland.ac.nz/SolidMechanicsBooks/Part_I/BookSM_Part_I/07_ElasticityApplications/07_Elasticity_Applications_03_Presure_Vessels.pdf
//         self.stress =
//             self.gage_pressure(external_pressure) * self.radius() / (2.0 * self.skin_thickness);
//         if self.stress > self.material.max_stress {
//             self.burst(format!(
//                 "Hoop stress ({:?} Pa) exceeded maximum stress ({:?} Pa)",
//                 self.stress, self.material.max_stress
//             ));
//         }
//     }

//     fn set_strain(&mut self) {
//         // strain (%) of thin-walled hollow sphere from internal pressure
//         // https://en.wikipedia.org/wiki/Pressure_vessel#Stress_in_thin-walled_pressure_vessels
//         // https://pkel015.connect.amazon.auckland.ac.nz/SolidMechanicsBooks/Part_I/BookSM_Part_I/07_ElasticityApplications/07_Elasticity_Applications_03_Presure_Vessels.pdf
//         self.strain = self.radius() / self.unstretched_radius;
//         if self.strain > self.material.max_strain {
//             self.burst(format!(
//                 "Tangential strain ({:?} %) exceeded maximum strain ({:?} %)",
//                 self.strain * 100.0,
//                 self.material.max_strain * 100.0
//             ));
//         }
//     }

//     pub fn radial_displacement(&self, external_pressure: f32) -> f32 {
//         // https://pkel015.connect.amazon.auckland.ac.nz/SolidMechanicsBooks/Part_I/BookSM_Part_I/07_ElasticityApplications/07_Elasticity_Applications_03_Presure_Vessels.pdf
//         ((1.0 - self.material.poissons_ratio) / self.material.elasticity)
//             * ((self.gage_pressure(external_pressure) * f32::powf(self.radius(), 2.0)) / 2.0
//                 * self.skin_thickness)
//     }

//     fn rebound(&mut self, radial_displacement: f32) -> f32 {
//         // https://physics.stackexchange.com/questions/10372/inflating-a-balloon-expansion-resistance
//         self.set_thickness(
//             self.unstretched_thickness * f32::powf(self.unstretched_radius / self.radius(), 2.0),
//         );
//         2.0 * self.material.elasticity
//             * radial_displacement
//             * self.unstretched_thickness
//             * self.unstretched_radius
//             / f32::powf(self.radius(), 3.0)
//     }

//     pub fn stretch(&mut self, external_pressure: f32) {
//         // stretch the balloon and/or compress the gas inside.
//         // - the gas wants to be at the same pressure as ambient
//         // - the balloon will stretch in response to the pressure difference
//         // - the balloon will likely not stretch enough to reach equilibrium
//         // - the difference between the ideal gas volume and the deformed
//         //   balloon volume is the new pressure difference
//         // - the balloon fails when it starts to plasticly deform, in other
//         //   words the balloon stretches as long as tangential stress is less
//         //   than the material's yield stress
//         debug!(
//             "current gage pressure: {:?}",
//             self.gage_pressure(external_pressure)
//         );

//         self.set_stress(external_pressure);
//         self.set_strain();

//         if self.intact {
//             let delta_r = self.radial_displacement(external_pressure);
//             debug!(
//                 "radius before stretch: {:?} delta_r: {:?}",
//                 self.radius(),
//                 delta_r
//             );
//             let internal_pressure = self.rebound(delta_r);
//             self.set_pressure(internal_pressure + external_pressure);
//             debug!("radius after stretch: {:?}", self.radius());
//             debug!(
//                 "gage pressure after stretch: {:?}",
//                 self.gage_pressure(external_pressure)
//             );
//         }
//     }

//     fn burst(&mut self, reason: String) {
//         // Assert new balloon attributes to reflect that it has burst
//         self.intact = false;
//         self.set_volume(0.0);
//         self.lift_gas.set_mass(0.0);
//         warn!("The balloon has burst! Reason: {:?}", reason)
//     }
// }
