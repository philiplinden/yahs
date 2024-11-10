//! Atmosphere model based on the US Standard Atmosphere, 1976.
//!
//! Reference:
//! - https://apps.dtic.mil/dtic/tr/fulltext/u2/a035728.pdf
//! - https://www.translatorscafe.com/unit-converter/en-US/calculator/altitude
//! - https://www.grc.nasa.gov/WWW/K-12/airplane/atmosmet.html

#![allow(dead_code)]

use bevy::prelude::*;

use crate::simulator::thermodynamics::{Density, MolarMass, Pressure, Temperature};

pub const ATMOSPHERE_MOLAR_MASS: f32 = 0.02897; // [kg/mol] molar mass of air

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
    pub const MOLAR_MASS: MolarMass = MolarMass(ATMOSPHERE_MOLAR_MASS);
    /// Temperature (K) of the atmosphere at a position.
    pub fn temperature(&self, position: Vec3) -> Temperature {
        // TODO: Look up temperature based on latitude, longitude, and altitude.
        coesa_temperature(position.y)
    }

    /// Pressure (Pa) of the atmosphere at a position.
    pub fn pressure(&self, position: Vec3) -> Pressure {
        // TODO: Look up pressure based on latitude, longitude, and altitude.
        coesa_pressure(position.y)
    }

    /// Density (kg/m³) of the atmosphere at a position.
    pub fn density(&self, position: Vec3) -> Density {
        Density::from_gas(
            self.temperature(position),
            self.pressure(position),
            Atmosphere::MOLAR_MASS,
        )
    }
}

/// Temperature (K) of the atmosphere at a given altitude (m).
/// Only valid for altitudes below 85,000 meters.
/// Based on the US Standard Atmosphere, 1976. (aka COESA)
fn coesa_temperature(altitude: f32) -> Temperature {
    if (-57.0..11000.0).contains(&altitude) {
        Temperature::from_celsius(15.04 - 0.00649 * altitude)
    } else if (11000.0..25000.0).contains(&altitude) {
        Temperature::from_celsius(-56.46)
    } else if (25000.0..85000.0).contains(&altitude) {
        Temperature::from_celsius(-131.21 + 0.00299 * altitude)
    } else {
        error!(
            "Altitude {:}m is outside of the accepted range! Must be 0-85000m",
            altitude
        );
        Temperature::from_celsius(0.0)
    }
}

/// Pressure (Pa) of the atmosphere at a given altitude (m).
/// Only valid for altitudes below 85,000 meters.
/// Based on the US Standard Atmosphere, 1976. (aka COESA)
fn coesa_pressure(altitude: f32) -> Pressure {
    if (-57.0..11000.0).contains(&altitude) {
        Pressure::from_kilopascal(
            101.29 * f32::powf(coesa_temperature(altitude).kelvin() / 288.08, 5.256),
        )
    } else if (11000.0..25000.0).contains(&altitude) {
        Pressure::from_kilopascal(22.65 * f32::exp(1.73 - 0.000157 * altitude))
    } else if (25000.0..85000.0).contains(&altitude) {
        Pressure::from_kilopascal(
            2.488 * f32::powf(coesa_temperature(altitude).kelvin() / 216.6, -11.388),
        )
    } else {
        error!(
            "Altitude {:}m is outside of the accepted range! Must be 0-85000m",
            altitude
        );
        Pressure(0.0)
    }
}
