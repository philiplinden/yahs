// ----------------------------------------------------------------------------
// Balloon
// -------
// Properties, attributes and functions related to the balloon.
// ----------------------------------------------------------------------------

extern crate libm;

use super::gas;
use super::materials;

use std::f32::consts::PI;

pub struct Balloon {
    pub intact: bool,              // whether or not it has burst
    pub mass: f32,                 // balloon mass (kg)
    pub temperature: f32,          // fail if surface temperature exceeds this (K)
    pub volume: f32,               // internal volume of the balloon (m^3)
    pub drag_coeff: f32,           // drag coefficient
    pub lift_gas: gas::GasVolume,  // gas inside the balloon
    material: materials::Material, // what the balloon is made of
    coating: materials::Material,  // what material is applied on top of the balloon
    initial_volume: f32,           // internal volume (m^3) at zero pressure
    balloon_thickness: f32,        // thickness of the skin of the balloon (m)
    pressure_difference: f32,      // pressure (Pa) in the balloon compared to ambient
}

impl Balloon {
    pub fn new(
        material: materials::Material, // material of balloon skin
        balloon_thickness: f32,        // balloon skin thickness (m) at zero pressure
        coating: materials::Material,  // surface coating of balloon skin
        coating_thickness: f32,        // balloon coating thickness
        barely_inflated_diameter: f32, // internal diameter (m) at zero pressure
        lift_gas: gas::GasVolume,      // species of gas inside balloon
    ) -> Self {
        let initial_radius = barely_inflated_diameter / 2.0;
        let initial_volume = spherical_volume(initial_radius);
        let bladder_mass = shell_volume(initial_radius, balloon_thickness) * material.density;
        let coating_mass =
            shell_volume(initial_radius + balloon_thickness, coating_thickness) * coating.density;
        Balloon {
            intact: true,
            mass: bladder_mass + coating_mass,
            temperature: 293.0,
            volume: initial_volume,
            pressure_difference: 0.0,
            drag_coeff: 0.3,
            lift_gas,
            material,
            coating,
            initial_volume,
            balloon_thickness,
        }
    }

    pub fn emissivity(self) -> f32 {
        if self.coating == materials::NOTHING {
            self.material.emissivity
        } else {
            self.coating.emissivity
        }
    }

    pub fn set_relative_pressure(&mut self, external_pressure: f32) {
        // pressure (Pa) in the balloon compared to ambient
        self.pressure_difference = self.lift_gas.pressure() - external_pressure;
        self.stretch()
    }

    fn stretch(&mut self) {
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
        equilibrium_gas.set_pressure(self.lift_gas.pressure() + self.pressure_difference);

        // percent elongation aka tangential strain (m/m)
        let original_radius = sphere_radius_from_volume(self.initial_volume);
        let equilibrium_radius = sphere_radius_from_volume(equilibrium_gas.volume());
        let elongation = (equilibrium_radius - original_radius) / original_radius;
        if elongation < self.material.max_elongation {
            self.volume = equilibrium_gas.volume();
            self.pressure_difference = 0.0;
        } else {
            self.volume = spherical_volume(original_radius * self.material.max_elongation);
            self.lift_gas.set_volume(self.volume);
            self.pressure_difference = equilibrium_gas.pressure() - self.lift_gas.pressure();
        }
        if tangential_stress(
            self.pressure_difference,
            self.volume,
            self.balloon_thickness,
        ) > self.material.max_stress
        {
            self.burst();
        }
    }

    fn burst(&mut self) {
        // Assert new balloon attributes to reflect that it has burst
        self.intact = false;
        self.volume = 0.0;
        self.pressure_difference = 0.0;
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
