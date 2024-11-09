//! Atmosphere model based on the US Standard Atmosphere, 1976.
#![allow(dead_code)]

use bevy::prelude::*;

use crate::simulator::{
    constants::ATMOSPHERE_MOLAR_MASS,
    gas::density,
    units::celsius2kelvin,
};

pub struct AtmospherePlugin;
impl Plugin for AtmospherePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Atmosphere);
    }
}

/// US Standard Atmosphere, 1976
#[derive(Resource)]
pub struct Atmosphere;

impl Atmosphere {
    /// Temperature (K) of the atmosphere at a position.
    pub fn temperature(&self, position: Vec3) -> f32 {
        // TODO: Look up temperature based on latitude, longitude, and altitude.
        coesa_temperature(position.y)
    }

    /// Pressure (Pa) of the atmosphere at a position.
    pub fn pressure(&self, position: Vec3) -> f32 {
        // TODO: Look up pressure based on latitude, longitude, and altitude.
        coesa_pressure(position.y)
    }

    /// Density (kg/m³) of the atmosphere at a position.
    pub fn density(&self, position: Vec3) -> f32 {
        density(
            self.temperature(position),
            self.pressure(position),
            ATMOSPHERE_MOLAR_MASS,
        )
    }
}

/// Temperature (K) of the atmosphere at a given altitude (m).
/// Only valid for altitudes below 85,000 meters.
/// Based on the US Standard Atmosphere, 1976. (aka COESA)
fn coesa_temperature(altitude: f32) -> f32 {
    if (-57.0..11000.0).contains(&altitude) {
        celsius2kelvin(15.04 - 0.00649 * altitude)
    } else if (11000.0..25000.0).contains(&altitude) {
        celsius2kelvin(-56.46)
    } else if (25000.0..85000.0).contains(&altitude) {
        celsius2kelvin(-131.21 + 0.00299 * altitude)
    } else {
        error!(
            "Altitude {:}m is outside of the accepted range! Must be 0-85000m",
            altitude
        );
        0.0
    }
}

/// Pressure (Pa) of the atmosphere at a given altitude (m).
/// Only valid for altitudes below 85,000 meters.
/// Based on the US Standard Atmosphere, 1976. (aka COESA)
fn coesa_pressure(altitude: f32) -> f32 {
    if (-57.0..11000.0).contains(&altitude) {
        (101.29 * f32::powf(coesa_temperature(altitude) / 288.08, 5.256)) * 1000.0
    } else if (11000.0..25000.0).contains(&altitude) {
        (22.65 * f32::exp(1.73 - 0.000157 * altitude)) * 1000.0
    } else if (25000.0..85000.0).contains(&altitude) {
        (2.488 * f32::powf(coesa_temperature(altitude) / 216.6, -11.388)) * 1000.0
    } else {
        error!(
            "Altitude {:}m is outside of the accepted range! Must be 0-85000m",
            altitude
        );
        0.0
    }
}
