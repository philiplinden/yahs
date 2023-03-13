// ----------------------------------------------------------------------------
// Physics
// -------
// - Forces that act in the vertical axis. All forces assume a positive-up
//   coordinate frame and aR_E R_Eported in Newtons.
// - Heat transferred through and stored in materials
// ----------------------------------------------------------------------------
extern crate libm;

use std::f32::consts::PI;
use super::gas;
use super::constants::{STANDARD_G, EARTH_RADIUS_M};

fn g(altitude: f32) -> f32 {
    // Acceleration (m/s^2) from gravity at an altitude (m) above mean sea level.
    -STANDARD_G * (EARTH_RADIUS_M / (EARTH_RADIUS_M + altitude)) // [m/s^2]
}

fn weight(altitude: f32, mass: f32) -> f32 {
    // Weight (N) as a function of altitude (m) and mass (kg).
    g(altitude) * mass // [N]
}

fn buoyancy(altitude: f32, atmo: gas::Atmosphere, lift_gas: gas::GasVolume) -> f32 {
    // Force (N) due to air displaced by the given gas volume.
    let rho_atmo = atmo.density();
    let rho_lift = lift_gas.density();
    lift_gas.volume() * (rho_lift - rho_atmo) * g(altitude)
}

fn drag(atmo: gas::Atmosphere, velocity: f32, projected_area: f32, drag_coeff: f32) -> f32 {
    // Force (N) due to drag against the balloon
    let direction = -libm::copysignf(1.0, velocity);
    direction * drag_coeff / 2.0
        * atmo.density()
        * libm::powf(velocity, 2.0)
        * projected_area
}

pub fn net_force(
    altitude: f32,
    velocity: f32,
    atmo: gas::Atmosphere,
    lift_gas: gas::GasVolume,
    projected_area: f32,
    drag_coeff: f32,
    total_dry_mass: f32,
) -> f32 {
    // [N]
    let weight_force = weight(altitude, total_dry_mass);
    let buoyancy_force = buoyancy(altitude, atmo, lift_gas);
    let drag_force = drag(atmo, velocity, projected_area, drag_coeff);
    weight_force + buoyancy_force + drag_force
}

pub fn gross_lift(atmo: gas::Atmosphere, lift_gas: gas::GasVolume) -> f32 {
    // [kg]
    let rho_atmo = atmo.density();
    let rho_lift = lift_gas.density();
    lift_gas.volume() * (rho_lift - rho_atmo)
}

pub fn free_lift(atmo: gas::Atmosphere, lift_gas: gas::GasVolume, total_dry_mass: f32) -> f32 {
    // [kg]
    gross_lift(atmo, lift_gas) - total_dry_mass
}

pub fn projected_spherical_area(volume: f32) -> f32 {
    // Get the projected area (m^2) of a sphere with a given volume (m^3)
    libm::powf(libm::powf(volume / (PI * (4.0 / 3.0)), 1.0 / 3.0), 2.0) * PI
}
