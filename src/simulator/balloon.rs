// ----------------------------------------------------------------------------
// Balloon
// -------
// Properties, attributes and functions related to the balloon.
// ----------------------------------------------------------------------------

extern crate libm;

use log::debug;
use serde::Deserialize;
use std::fmt;

use super::gas;
use super::geometry::{
    shell_volume, sphere_radius_from_volume, sphere_surface_area, sphere_volume,
};

#[derive(Copy, Clone, PartialEq)]
pub enum BalloonStatus {
    Underinflated,
    Ok,
    Burst,
}

#[derive(Copy, Clone)]
pub struct Balloon {
    pub status: BalloonStatus,    // status of the balloon
    pub mass: f32,                // balloon mass (kg)
    pub temperature: f32,         // temperature of the balloon skin (K)
    pub drag_coeff: f32,          // drag coefficient
    pub lift_gas: gas::GasVolume, // gas inside the balloon
    pub material: Material,       // what the balloon is made of
    pub skin_thickness: f32,      // thickness of the skin of the balloon (m)
    unstretched_radius: f32,      // radius of balloon without stretch (m)
    stress: f32,
    strain: f32,
}

impl Balloon {
    pub fn new(
        material: Material,            // material of balloon skin
        barely_inflated_diameter: f32, // internal diameter (m) at zero pressure
        lift_gas: gas::GasVolume,      // species of gas inside balloon
    ) -> Self {
        let skin_thickness = material.unloaded_thickness;
        let unstretched_radius = barely_inflated_diameter / 2.0;
        let mass = shell_volume(unstretched_radius, skin_thickness) * material.density;
        let status: BalloonStatus;
        let unconstrained_volume = lift_gas.volume();
        let base_volume = sphere_volume(unstretched_radius);
        if unconstrained_volume < base_volume {
            status = BalloonStatus::Underinflated;
        } else {
            status = BalloonStatus::Ok;
        }
        Balloon {
            status,
            mass,
            temperature: 293.0,
            drag_coeff: 0.3,
            lift_gas,
            material,
            skin_thickness,
            unstretched_radius,
            stress: 0.0,
            strain: 0.0,
        }
    }

    pub fn surface_area(self) -> f32 {
        sphere_surface_area(sphere_radius_from_volume(self.lift_gas.volume()))
    }

    pub fn radius(self) -> f32 {
        sphere_radius_from_volume(self.volume())
    }

    pub fn set_radius(&mut self, new_radius: f32) {
        self.set_volume(sphere_volume(new_radius))
    }

    pub fn volume(self) -> f32 {
        self.lift_gas.volume()
    }

    fn set_volume(&mut self, new_volume: f32) {
        self.lift_gas.set_volume(new_volume)
    }

    pub fn pressure(&mut self) -> f32 {
        self.lift_gas.pressure()
    }

    fn set_pressure(&mut self, new_pressure: f32) {
        self.lift_gas.set_pressure(new_pressure)
    }

    fn set_thickness(&mut self, new_thickness: f32) {
        self.skin_thickness = new_thickness
    }

    pub fn stress(self) -> f32 {
        self.stress
    }

    fn set_stress(&mut self, gage_pressure: f32) {
        // hoop stress (Pa) of thin-walled hollow sphere from internal pressure
        // https://en.wikipedia.org/wiki/Pressure_vessel#Stress_in_thin-walled_pressure_vessels
        // https://pkel015.connect.amazon.auckland.ac.nz/SolidMechanicsBooks/Part_I/BookSM_Part_I/07_ElasticityApplications/07_Elasticity_Applications_03_Presure_Vessels.pdf
        self.stress =
            gage_pressure * self.radius() / (2.0 * self.skin_thickness);
        if self.stress > self.material.max_stress {
            self.burst(Some(format!(
                "Hoop stress ({:?} Pa) exceeded maximum stress ({:?} Pa)",
                self.stress, self.material.max_stress
            )));
        }
    }

    pub fn strain(self) -> f32 {
        self.strain
    }

    fn set_strain(&mut self) {
        // strain (%) of thin-walled hollow sphere from internal pressure
        // https://en.wikipedia.org/wiki/Pressure_vessel#Stress_in_thin-walled_pressure_vessels
        // https://pkel015.connect.amazon.auckland.ac.nz/SolidMechanicsBooks/Part_I/BookSM_Part_I/07_ElasticityApplications/07_Elasticity_Applications_03_Presure_Vessels.pdf
        self.strain = (self.radius() / self.unstretched_radius) - 1.0;
        if self.strain > self.material.max_strain {
            self.burst(Some(format!(
                "Tangential strain ({:?} %) exceeded maximum strain ({:?} %)",
                self.strain * 100.0,
                self.material.max_strain * 100.0
            )));
        }
    }

    fn radial_displacement(self, gage_pressure: f32) -> f32 {
        // https://pkel015.connect.amazon.auckland.ac.nz/SolidMechanicsBooks/Part_I/BookSM_Part_I/07_ElasticityApplications/07_Elasticity_Applications_03_Presure_Vessels.pdf
        ((1.0 - self.material.poissons_ratio) / self.material.elastic_modulus)
            * ((gage_pressure * libm::powf(self.radius(), 2.0)) / 2.0
                * self.skin_thickness)
    }

    fn rebound(&mut self, radial_displacement: f32) -> f32 {
        // https://physics.stackexchange.com/questions/10372/inflating-a-balloon-expansion-resistance
        self.set_thickness(
            self.material.unloaded_thickness
                * libm::powf(self.unstretched_radius / self.radius(), 2.0),
        );
        2.0 * self.material.elastic_modulus
            * radial_displacement
            * self.skin_thickness
            * self.unstretched_radius
            / libm::powf(self.radius(), 3.0)
    }

    pub fn stretch(&mut self, external_pressure: f32) {
        // stretch the balloon and/or compress the gas inside.
        // - the gas wants to be at the same pressure as ambient
        // - the balloon will stretch in response to the pressure difference
        // - the balloon will likely not stretch enough to reach equilibrium
        // - the difference between the ideal gas volume and the deformed
        //   balloon volume is the new pressure difference
        // - the balloon fails when it starts to plasticly deform, in other
        //   words the balloon stretches as long as tangential stress is less
        //   than the material's yield stress
        let gage_pressure = self.lift_gas.pressure() - external_pressure;
        debug!(
            "gage pressure before stretch: {:?}",
            gage_pressure
        );

        self.set_stress(gage_pressure);
        self.set_strain();

        if self.status != BalloonStatus::Burst {
            let delta_r = self.radial_displacement(gage_pressure);
            debug!(
                "radius before stretch: {:?} delta_r: {:?}",
                self.radius(),
                delta_r
            );
            let internal_pressure = self.rebound(delta_r);
            self.set_pressure(internal_pressure + external_pressure);
            debug!("radius after stretch: {:?}", self.radius());
        }
    }

    fn burst(&mut self, reason: Option<String>) {
        // Assert new balloon attributes to reflect that it has burst
        self.status = BalloonStatus::Burst;
        self.set_volume(0.0);
        self.lift_gas.set_mass(0.0);

        match reason {
            Some(r) => log::info!("The balloon has burst! Reason: {r}"),
            None => log::info!("The balloon has burst!"),
        }
    }
}

// ----------------------------------------------------------------------------
// Materials
// ---------
// Properties, attributes and functions related to the material properties.
// Source: https://www.matweb.com/
// ----------------------------------------------------------------------------

#[derive(Copy, Clone, PartialEq)]
pub struct Material {
    pub unloaded_thickness: f32,   // thickness of material (m) with zero load
    pub max_temperature: f32,      // temperature (K) where the given material fails
    pub density: f32,              // density (kg/m^3)
    pub emissivity: f32,           // how much thermal radiation is emitted
    pub absorptivity: f32,         // how much thermal radiation is absorbed
    pub thermal_conductivity: f32, // thermal conductivity (W/mK) of the material at room temperature
    pub specific_heat: f32,        // J/kgK
    pub poissons_ratio: f32,       // ratio of change in width for a given change in length
    pub elastic_modulus: f32,      // Youngs Modulus aka Modulus of Elasticity (Pa)
    pub max_strain: f32,           // elongation at failure (decimal, unitless) 1 = original size
    pub max_stress: f32,           // tangential stress at failure (Pa)
}

impl Material {
    pub fn new(material_type: MaterialType) -> Self {
        match material_type {
            MaterialType::Magic => MAGIC,
            MaterialType::Rubber => RUBBER,
            MaterialType::LDPE | MaterialType::LowDensityPolyethylene => LOW_DENSITY_POLYETHYLENE,
            _ => NOTHING,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Deserialize)]
pub enum MaterialType {
    // Species of gas with a known molar mass (kg/mol)
    Nothing,
    Magic,
    Rubber,
    LDPE,
    LowDensityPolyethylene,
}

impl fmt::Display for MaterialType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MaterialType::Nothing => write!(f, "nothing"),
            MaterialType::Magic => write!(f, "magic"),
            MaterialType::Rubber => write!(f, "rubber"),
            MaterialType::LDPE | MaterialType::LowDensityPolyethylene => {
                write!(f, "low-density polyethylene (LDPE)")
            }
        }
    }
}

pub const NOTHING: Material = Material {
    // nothing
    unloaded_thickness: 0.0,
    max_temperature: f32::INFINITY,
    density: 0.0,
    emissivity: 1.0,
    absorptivity: 0.0,
    thermal_conductivity: f32::INFINITY,
    specific_heat: 0.0,
    poissons_ratio: 0.5,
    elastic_modulus: f32::INFINITY,
    max_strain: f32::INFINITY,
    max_stress: f32::INFINITY,
};

pub const MAGIC: Material = Material {
    // an imaginary material with arbitrary properties (for testing only)
    unloaded_thickness: 0.000_000_1,
    max_temperature: 500.0,
    density: 0.01,
    emissivity: 1.0,
    absorptivity: 0.0,
    thermal_conductivity: 1.0,
    specific_heat: 1.0,
    poissons_ratio: 0.8,
    elastic_modulus: 1_000_000_000.0,
    max_strain: f32::INFINITY,
    max_stress: f32::INFINITY,
};

pub const RUBBER: Material = Material {
    // Nitrile Butadiene Rubber
    // https://designerdata.nl/materials/plastics/rubbers/nitrile-butadiene-rubber
    unloaded_thickness: 0.000_002,
    max_temperature: 385.0,
    density: 1000.0,
    emissivity: 0.86,
    absorptivity: 0.86,
    thermal_conductivity: 0.25,
    specific_heat: 1490.0,
    poissons_ratio: 0.8,
    elastic_modulus: 4_000_000.0,
    max_strain: 8.0,
    max_stress: 25_000_000.0,
};

pub const LOW_DENSITY_POLYETHYLENE: Material = Material {
    // Low Density Polyethylene (LDPE)
    // https://designerdata.nl/materials/plastics/thermo-plastics/low-density-polyethylene
    unloaded_thickness: 0.000_002,
    max_temperature: 348.0,
    density: 919.0,
    emissivity: 0.94,
    absorptivity: 0.94,
    thermal_conductivity: 0.3175,
    specific_heat: 2600.0,
    poissons_ratio: 0.8,
    elastic_modulus: 300_000_000.0,
    max_strain: 6.25,
    max_stress: 10_000_000.0,
};
