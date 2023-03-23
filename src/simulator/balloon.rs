// ----------------------------------------------------------------------------
// Balloon
// -------
// Properties, attributes and functions related to the balloon.
// ----------------------------------------------------------------------------

extern crate libm;

use serde::Deserialize;
use std::f32::consts::PI;
use std::fmt;

use super::gas;

pub struct Balloon {
    pub intact: bool,              // whether or not it has burst
    pub mass: f32,                 // balloon mass (kg)
    pub temperature: f32,          // fail if surface temperature exceeds this (K)
    pub volume: f32,               // internal volume of the balloon (m^3)
    pub drag_coeff: f32,           // drag coefficient
    pub lift_gas: gas::GasVolume,  // gas inside the balloon
    material: Material, // what the balloon is made of
    initial_volume: f32,           // internal volume (m^3) at zero pressure
    skin_thickness: f32,        // thickness of the skin of the balloon (m)
}

impl Balloon {
    pub fn new(
        material: Material, // material of balloon skin
        skin_thickness: f32,        // balloon skin thickness (m) at zero pressure
        barely_inflated_diameter: f32, // internal diameter (m) at zero pressure
        lift_gas: gas::GasVolume,      // species of gas inside balloon
    ) -> Self {
        let initial_radius = barely_inflated_diameter / 2.0;
        let initial_volume = spherical_volume(initial_radius);
        let mass = shell_volume(initial_radius, skin_thickness) * material.density;
        Balloon {
            intact: true,
            mass,
            temperature: 293.0,
            volume: initial_volume,
            drag_coeff: 0.3,
            lift_gas,
            material,
            initial_volume,
            skin_thickness,
        }
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

        let mut equilibrium_gas = self.lift_gas.clone();
        equilibrium_gas.set_pressure(external_pressure);

        // percent elongation aka tangential strain (m/m)
        let original_radius = sphere_radius_from_volume(self.initial_volume);
        let equilibrium_radius = sphere_radius_from_volume(equilibrium_gas.volume());
        let elongation = (equilibrium_radius - original_radius) / original_radius;
        if elongation < self.material.max_elongation {
            self.volume = self.lift_gas.volume();
            self.lift_gas.set_pressure(external_pressure);
        } else {
            self.burst()
            // self.volume = spherical_volume(original_radius * self.material.max_elongation);
            // self.lift_gas.set_volume(self.volume);
        }
        // let stress = tangential_stress(
        //     self.lift_gas.pressure() - external_pressure,
        //     self.volume,
        //     self.balloon_thickness,
        // );
        // if stress > self.material.max_stress
        // {
        //     self.burst();
        // }
    }

    fn burst(&mut self) {
        // Assert new balloon attributes to reflect that it has burst
        self.intact = false;
        self.volume = 0.0;
        self.lift_gas.set_mass(0.0);
    }
}

fn spherical_volume(radius: f32) -> f32 {
    (4.0 / 3.0) * PI * libm::powf(radius, 3.0)
}

fn shell_volume(internal_radius: f32, thickness: f32) -> f32 {
    let external_radius = internal_radius + thickness;
    let internal_volume = spherical_volume(internal_radius);
    let external_volume = spherical_volume(external_radius);
    external_volume - internal_volume
}

fn sphere_radius_from_volume(volume: f32) -> f32 {
    libm::powf(volume, 1.0 / 3.0) / (4.0 / 3.0) * PI
}

fn tangential_stress(pressure_difference: f32, internal_volume: f32, shell_thickness: f32) -> f32 {
    // tangential stress (Pa) of hollow sphere from internal pressure
    pressure_difference * sphere_radius_from_volume(internal_volume) / (2.0 * shell_thickness)
}

// ----------------------------------------------------------------------------
// Materials
// ---------
// Properties, attributes and functions related to the material properties.
// Source: https://www.matweb.com/
// ----------------------------------------------------------------------------

#[derive(Clone, PartialEq)]
pub struct Material {
    pub max_temperature: f32, // temperature (K) where the given material fails
    pub density: f32, // density (kg/m^3)
    pub emissivity: f32, // emissivity coefficient of the material for blackbody light
    pub thermal_conductivity: f32, // thermal conductivity (W/mK) of the material at room temperature
    pub max_elongation: f32, // elongation at failure (decimal, unitless) 1 = original size
    pub max_stress: f32, // tangential stress at failure (Pa)
}

impl Material {
    pub fn new(material_type: MaterialType) -> Self {
        match material_type {
            MaterialType::Rubber => RUBBER,
            MaterialType::LowDensityPolyethylene => LOW_DENSITY_POLYETHYLENE,
            _ => NOTHING
        }
    }
}

#[derive(Copy, Clone, PartialEq, Deserialize)]
pub enum MaterialType {
    // Species of gas with a known molar mass (kg/mol)
    Nothing,
    Rubber,
    LowDensityPolyethylene,
}

impl fmt::Display for MaterialType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MaterialType::Nothing => write!(f, "nothing"),
            MaterialType::Rubber => write!(f, "rubber"),
            MaterialType::LowDensityPolyethylene => write!(f, "low-density polyethylene (LDPE)"),
        }
    }
}

pub const NOTHING: Material = Material {
    // nothing
    max_temperature: f32::INFINITY,
    density: 0.0,
    emissivity: 1.0,
    thermal_conductivity: f32::INFINITY,
    max_elongation: f32::INFINITY,
    max_stress: f32::INFINITY,
};

pub const RUBBER: Material = Material {
    // Natural Rubber, Vulcanized (NR, IR, Polyisoprene)
    max_temperature: 400.0,
    density: 950.0,
    emissivity: 0.86,
    thermal_conductivity: 0.34,
    max_elongation: 8.0,
    max_stress: 150_000_000.0,
};

pub const LOW_DENSITY_POLYETHYLENE: Material = Material {
    // Low Density Polyethylene (LDPE), Film Grade
    max_temperature: 380.0,
    density: 910.0,
    emissivity: 0.94,
    thermal_conductivity: 0.15,
    max_elongation: 1.0,
    max_stress: 300_000_000.0,
};