use bevy::prelude::*;
use std::fmt;

use crate::simulator::{
    constants::*,
    gas::density,
    units::celsius2kelvin,
};

#[derive(Copy, Clone)]
pub struct Atmosphere {
    // US Standard Atmosphere, 1976
    altitude: f32,    // [m] altitude (which determines the other attributes)
    temperature: f32, // [K] temperature
    pressure: f32,    // [Pa] pressure
    density: f32,     // [kg/m³] density
    molar_mass: f32,  // [kg/mol] molar mass a.k.a. molecular weight
}

impl fmt::Display for Atmosphere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:} K | {:} Pa | {:} kg/m³",
            self.temperature, self.pressure, self.density,
        )
    }
}

impl Atmosphere {
    pub fn new(altitude: f32) -> Self {
        Atmosphere {
            altitude,
            temperature: coesa_temperature(altitude),
            pressure: coesa_pressure(altitude),
            density: density(
                coesa_temperature(altitude),
                coesa_pressure(altitude),
                28.9647,
            ),
            molar_mass: 28.9647,
        }
    }
    pub fn set_altitude(&mut self, new_altitude: f32) {
        self.altitude = new_altitude;
        // update all params
        self.temperature = coesa_temperature(new_altitude);
        self.pressure = coesa_pressure(new_altitude);
        self.density = density(self.temperature, self.pressure, self.molar_mass);
    }

    pub fn temperature(self) -> f32 {
        // Temperature (K)
        self.temperature
    }

    pub fn pressure(self) -> f32 {
        // Pressure (Pa)
        self.pressure
    }

    pub fn density(self) -> f32 {
        // Density (kg/m³)
        self.density
    }
}

impl Default for Atmosphere {
    fn default() -> Self {
        Atmosphere {
            altitude: 0.0, // Sea level
            temperature: STANDARD_TEMPERATURE, // Standard sea level temperature
            pressure: STANDARD_PRESSURE, // Standard sea level pressure
            density: density(STANDARD_TEMPERATURE, STANDARD_PRESSURE, 28.9647), // Calculate density at sea level
            molar_mass: 28.9647, // Molar mass of air
        }
    }
}

fn coesa_temperature(altitude: f32) -> f32 {
    // Temperature (K) of the atmosphere at a given altitude (m).
    // Only valid for altitudes below 85,000 meters.
    // Based on the US Standard Atmosphere, 1976. (aka COESA)
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

fn coesa_pressure(altitude: f32) -> f32 {
    // Pressure (Pa) of the atmosphere at a given altitude (m).
    // Only valid for altitudes below 85,000 meters.
    // Based on the US Standard Atmosphere, 1976. (aka COESA)
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
