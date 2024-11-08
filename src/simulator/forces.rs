// ----------------------------------------------------------------------------
// Forces
// -------
// Forces that act in the vertical axis. All forces assume a positive-up
// coordinate frame and aR_E R_Eported in Newtons.
// ----------------------------------------------------------------------------

use super::constants::{EARTH_RADIUS_M, STANDARD_G};
use super::{gas, atmosphere, SolidBody};

fn g(altitude: f32) -> f32 {
    // Acceleration (m/s^2) from gravity at an altitude (m) above mean sea level.
    -STANDARD_G * (EARTH_RADIUS_M / (EARTH_RADIUS_M + altitude)) // [m/s^2]
}

pub fn weight(altitude: f32, mass: f32) -> f32 {
    // Weight (N) as a function of altitude (m) and mass (kg).
    g(altitude) * mass // [N]
}

pub fn buoyancy(altitude: f32, atmo: atmosphere::Atmosphere, lift_gas: gas::GasVolume) -> f32 {
    // Force (N) due to air displaced by the given gas volume.
    let v = lift_gas.volume();
    if v > 0.0 {
        let rho_atmo = atmo.density();
        let rho_lift = lift_gas.density();
        return lift_gas.volume() * (rho_lift - rho_atmo) * g(altitude)
    } else {
        return 0.0
    }
}

pub fn drag<T: SolidBody>(atmo: atmosphere::Atmosphere, velocity: f32, body: T) -> f32 {
    // Force (N) due to drag against the balloon
    let direction = -f32::copysign(1.0, velocity);
    direction * body.drag_coeff() / 2.0 * atmo.density() * f32::powf(velocity, 2.0) * body.drag_area()
}

pub fn gross_lift(atmo: atmosphere::Atmosphere, lift_gas: gas::GasVolume) -> f32 {
    // [kg]
    let rho_atmo = atmo.density();
    let rho_lift = lift_gas.density();
    lift_gas.volume() * (rho_lift - rho_atmo)
}

pub fn free_lift(atmo: atmosphere::Atmosphere, lift_gas: gas::GasVolume, total_dry_mass: f32) -> f32 {
    // [kg]
    gross_lift(atmo, lift_gas) - total_dry_mass
}
