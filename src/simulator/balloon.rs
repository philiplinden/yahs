//! Properties, attributes and functions related to the balloon.
#![allow(dead_code)]

use serde::Deserialize;
use bevy::prelude::*;

use crate::simulator::thermodynamics::IdealGas;

pub struct BalloonPlugin;

impl Plugin for BalloonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (simple_scene, spawn_balloon));

        // Register types for reflection
        app.register_type::<Balloon>();
        app.register_type::<BalloonMaterial>();
    }
}

#[derive(Bundle)]
pub struct BalloonBundle {
    pub balloon: Balloon,
    pub gas: IdealGas,
    pub transform: TransformBundle,
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

#[derive(Component, Reflect)]
pub struct Balloon {
    /// Balloon material type
    pub skin_material: BalloonMaterial,
    /// Thickness of balloon membrane in meters
    pub unstretched_thickness: f32,
    /// radius of balloon without stretch (m)
    pub unstretched_radius: f32,
}

fn spawn_balloon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {

    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(1.0)),
        material: materials.add(Color::srgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    commands.spawn(BalloonBundle {
        balloon: Balloon {
            skin_material: BalloonMaterial::default(),
            unstretched_thickness: 0.001,
            unstretched_radius: 1.0,
        },
        gas: IdealGas::default(),
        transform: TransformBundle::default(),
    });
}


/// set up a simple 3D scene
fn simple_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

// impl Balloon {

//     pub fn surface_area(&self) -> f32 {
//         sphere_surface_area(sphere_radius_from_volume(self.lift_gas.volume()))
//     }

//     pub fn radius(&self) -> f32 {
//         sphere_radius_from_volume(self.volume())
//     }

//     pub fn volume(&self) -> f32 {
//         self.lift_gas.volume()
//     }

//     fn set_volume(&mut self, new_volume: f32) {
//         self.lift_gas.set_volume(new_volume)
//     }

//     pub fn pressure(&self) -> f32 {
//         self.lift_gas.pressure()
//     }

//     fn set_pressure(&mut self, new_pressure: f32) {
//         self.lift_gas.set_pressure(new_pressure)
//     }

//     fn set_thickness(&mut self, new_thickness: f32) {
//         self.skin_thickness = new_thickness
//     }

//     pub fn gage_pressure(&self, external_pressure: f32) -> f32 {
//         self.lift_gas.pressure() - external_pressure
//     }

//     pub fn stress(&self) -> f32 {
//         self.stress
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

//     pub fn strain(&self) -> f32 {
//         self.strain
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

// impl<'a> SolidBody for Balloon<'a> {
//     fn drag_area(&self) -> f32 {
//         if self.intact {
//             sphere_radius_from_volume(self.volume())
//         } else {
//             0.0
//         }
//     }

//     fn drag_coeff(&self) -> f32 {
//         self.drag_coeff
//     }
// }

// fn sphere_volume(radius: f32) -> f32 {
//     (4.0 / 3.0) * PI * f32::powf(radius, 3.0)
// }

// fn shell_volume(internal_radius: f32, thickness: f32) -> f32 {
//     let external_radius = internal_radius + thickness;
//     let internal_volume = sphere_volume(internal_radius);
//     let external_volume = sphere_volume(external_radius);
//     external_volume - internal_volume
// }

// fn sphere_radius_from_volume(volume: f32) -> f32 {
//     f32::powf(volume, 1.0 / 3.0) / (4.0 / 3.0) * PI
// }

// fn sphere_surface_area(radius: f32) -> f32 {
//     4.0 * PI * f32::powf(radius, 2.0)
// }

// pub fn projected_spherical_area(volume: f32) -> f32 {
//     // Get the projected area (m^2) of a sphere with a given volume (m³)
//     f32::powf(sphere_radius_from_volume(volume), 2.0) * PI
// }
