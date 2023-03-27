// ----------------------------------------------------------------------------
// Balloon
// -------
// Properties, attributes and functions related to the balloon.
// ----------------------------------------------------------------------------

extern crate libm;

use log::debug;
use serde::Deserialize;
use std::f32::consts::PI;
use std::fmt;

use super::gas;

#[derive(Copy, Clone)]
pub struct Balloon {
    pub intact: bool,             // whether or not it has burst
    pub mass: f32,                // balloon mass (kg)
    pub temperature: f32,         // fail if surface temperature exceeds this (K)
    pub drag_coeff: f32,          // drag coefficient
    pub lift_gas: gas::GasVolume, // gas inside the balloon
    pub material: Material,       // what the balloon is made of
    pub skin_thickness: f32,      // thickness of the skin of the balloon (m)
    unstretched_thickness: f32,   // thickness of the skin of the balloon without stretch (m)
    unstretched_radius: f32,      // radius of balloon without stretch (m)
    stress: f32,
    strain: f32,
}

impl Balloon {
    pub fn new(
        material: Material,            // material of balloon skin
        skin_thickness: f32,           // balloon skin thickness (m) at zero pressure
        barely_inflated_diameter: f32, // internal diameter (m) at zero pressure
        lift_gas: gas::GasVolume,      // species of gas inside balloon
    ) -> Self {
        let unstretched_radius = barely_inflated_diameter / 2.0;
        let mass = shell_volume(unstretched_radius, skin_thickness) * material.density;
        Balloon {
            intact: true,
            mass,
            temperature: 293.0,
            drag_coeff: 0.3,
            lift_gas,
            material,
            skin_thickness,
            unstretched_thickness: skin_thickness,
            unstretched_radius,
            stress: 0.0,
            strain: 1.0,
        }
    }

    pub fn surface_area(self) -> f32 {
        sphere_surface_area(sphere_radius_from_volume(self.lift_gas.volume()))
    }

    pub fn radius(self) -> f32 {
        sphere_radius_from_volume(self.volume())
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

    fn gage_pressure(self, external_pressure: f32) -> f32 {
        self.lift_gas.pressure() - external_pressure
    }

    pub fn stress(self) -> f32 {
        self.stress
    }

    fn set_stress(&mut self, external_pressure: f32) {
        // hoop stress (Pa) of thin-walled hollow sphere from internal pressure
        // https://en.wikipedia.org/wiki/Pressure_vessel#Stress_in_thin-walled_pressure_vessels
        // https://pkel015.connect.amazon.auckland.ac.nz/SolidMechanicsBooks/Part_I/BookSM_Part_I/07_ElasticityApplications/07_Elasticity_Applications_03_Presure_Vessels.pdf
        self.stress = self.gage_pressure(external_pressure) * self.radius() / (2.0 * self.skin_thickness);
        if self.stress > self.material.max_stress {
            self.burst(format!(
                "Hoop stress ({:?} Pa) exceeded maximum stress ({:?} Pa)",
                self.stress, self.material.max_stress
            ));
        }
    }

    pub fn strain(self) -> f32 {
        self.strain
    }

    fn set_strain(&mut self) {
        // strain (%) of thin-walled hollow sphere from internal pressure
        // https://en.wikipedia.org/wiki/Pressure_vessel#Stress_in_thin-walled_pressure_vessels
        // https://pkel015.connect.amazon.auckland.ac.nz/SolidMechanicsBooks/Part_I/BookSM_Part_I/07_ElasticityApplications/07_Elasticity_Applications_03_Presure_Vessels.pdf
        self.strain = self.radius() / self.unstretched_radius;
        if self.strain > self.material.max_strain {
            self.burst(format!(
                "Tangential strain ({:?} %) exceeded maximum strain ({:?} %)",
                self.strain * 100.0,
                self.material.max_strain * 100.0
            ));
        }
    }

    fn radial_displacement(self, external_pressure: f32) -> f32 {
        // https://pkel015.connect.amazon.auckland.ac.nz/SolidMechanicsBooks/Part_I/BookSM_Part_I/07_ElasticityApplications/07_Elasticity_Applications_03_Presure_Vessels.pdf
        ((1.0 - self.material.poissons_ratio) / self.material.elasticity)
            * ((self.gage_pressure(external_pressure) * libm::powf(self.radius(), 2.0)) / 2.0
                * self.skin_thickness)
    }

    fn rebound(&mut self, radial_displacement: f32) -> f32 {
        // https://physics.stackexchange.com/questions/10372/inflating-a-balloon-expansion-resistance
        self.set_thickness(
            self.unstretched_thickness * libm::powf(self.unstretched_radius / self.radius(), 2.0),
        );
        2.0 * self.material.elasticity
            * radial_displacement
            * self.unstretched_thickness
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
        debug!(
            "current gage pressure: {:?}",
            self.gage_pressure(external_pressure)
        );

        self.set_stress(external_pressure);
        self.set_strain();

        if self.intact {
            let delta_r = self.radial_displacement(external_pressure);
            debug!(
                "radius before stretch: {:?} delta_r: {:?}",
                self.radius(),
                delta_r
            );
            let internal_pressure = self.rebound(delta_r);
            self.set_pressure(internal_pressure + external_pressure);
            debug!("radius after stretch: {:?}", self.radius());
            debug!(
                "gage pressure after stretch: {:?}",
                self.gage_pressure(external_pressure)
            );
        }

    }

    fn burst(&mut self, reason: String) {
        // Assert new balloon attributes to reflect that it has burst
        self.intact = false;
        self.set_volume(0.0);
        self.lift_gas.set_mass(0.0);
        log::warn!("The balloon has burst! Reason: {:?}", reason)
    }
}

fn sphere_volume(radius: f32) -> f32 {
    (4.0 / 3.0) * PI * libm::powf(radius, 3.0)
}

fn shell_volume(internal_radius: f32, thickness: f32) -> f32 {
    let external_radius = internal_radius + thickness;
    let internal_volume = sphere_volume(internal_radius);
    let external_volume = sphere_volume(external_radius);
    external_volume - internal_volume
}

fn sphere_radius_from_volume(volume: f32) -> f32 {
    libm::powf(volume, 1.0 / 3.0) / (4.0 / 3.0) * PI
}

fn sphere_surface_area(radius: f32) -> f32 {
    4.0 * PI * libm::powf(radius, 2.0)
}

// ----------------------------------------------------------------------------
// Materials
// ---------
// Properties, attributes and functions related to the material properties.
// Source: https://www.matweb.com/
// ----------------------------------------------------------------------------

#[derive(Copy, Clone, PartialEq)]
pub struct Material {
    pub max_temperature: f32, // temperature (K) where the given material fails
    pub density: f32,         // density (kg/m^3)
    pub emissivity: f32,      // how much thermal radiation is emitted
    pub absorptivity: f32,    // how much thermal radiation is absorbed
    pub thermal_conductivity: f32, // thermal conductivity (W/mK) of the material at room temperature
    pub specific_heat: f32,        // J/kgK
    pub poissons_ratio: f32,       // ratio of change in width for a given change in length
    pub elasticity: f32,           // Youngs Modulus aka Modulus of Elasticity (Pa)
    pub max_strain: f32,           // elongation at failure (decimal, unitless) 1 = original size
    pub max_stress: f32,           // tangential stress at failure (Pa)
}

impl Material {
    pub fn new(material_type: MaterialType) -> Self {
        match material_type {
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
    Rubber,
    LDPE,
    LowDensityPolyethylene,
}

impl fmt::Display for MaterialType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MaterialType::Nothing => write!(f, "nothing"),
            MaterialType::Rubber => write!(f, "rubber"),
            MaterialType::LDPE | MaterialType::LowDensityPolyethylene => {
                write!(f, "low-density polyethylene (LDPE)")
            }
        }
    }
}

pub const NOTHING: Material = Material {
    // nothing
    max_temperature: f32::INFINITY,
    density: 0.0,
    emissivity: 1.0,
    absorptivity: 0.0,
    thermal_conductivity: f32::INFINITY,
    specific_heat: 0.0,
    poissons_ratio: 0.5,
    elasticity: f32::INFINITY,
    max_strain: f32::INFINITY,
    max_stress: f32::INFINITY,
};

pub const RUBBER: Material = Material {
    // Nitrile Butadiene Rubber
    // https://designerdata.nl/materials/plastics/rubbers/nitrile-butadiene-rubber
    max_temperature: 385.0,
    density: 1000.0,
    emissivity: 0.86,
    absorptivity: 0.86,
    thermal_conductivity: 0.25,
    specific_heat: 1490.0,
    poissons_ratio: 0.5,
    elasticity: 4_000_000.0,
    // max_strain: 8.0,
    max_strain: 8.0,
    max_stress: 25_000_000.0,
};

pub const LOW_DENSITY_POLYETHYLENE: Material = Material {
    // Low Density Polyethylene (LDPE)
    // https://designerdata.nl/materials/plastics/thermo-plastics/low-density-polyethylene
    max_temperature: 348.0,
    density: 919.0,
    emissivity: 0.94,
    absorptivity: 0.94,
    thermal_conductivity: 0.3175,
    specific_heat: 2600.0,
    poissons_ratio: 0.5,
    elasticity: 300_000_000.0,
    max_strain: 6.25,
    max_stress: 10_000_000.0,
};
