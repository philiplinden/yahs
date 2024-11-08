// ----------------------------------------------------------------------------
// Gas
// ---
// Ideal gas equations. This model assumes that temperature and pressure are
// known, and that volume is not constrained. As such, only a gas's species,
// mass, temperature, and pressure can be set explicitly; volume and density
// are read-only derived attributes.
// ----------------------------------------------------------------------------
// Atmosphere
// ----------
// Approximate atmospheric temperature and pressure as a function of altitude.
// Based on the US Standard Atmosphere, 1976. (aka COESA)
// Reference:
//  https://apps.dtic.mil/dtic/tr/fulltext/u2/a035728.pdf
//  https://www.translatorscafe.com/unit-converter/en-US/calculator/altitude
//  https://www.grc.nasa.gov/WWW/K-12/airplane/atmosmet.html
// ----------------------------------------------------------------------------

#![allow(dead_code)]

use super::constants::{R, STANDARD_PRESSURE, STANDARD_TEMPERATURE};
use log::error;
use ron::de::from_str;
use serde::Deserialize;
use std::fmt;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct GasSpecies {
    pub name: String,
    pub abbreviation: String,
    pub molar_mass: f32, // [kg/mol] molar mass a.k.a. molecular weight
}

impl GasSpecies {
    pub fn new(name: String, abbreviation: String, molar_mass: f32) -> Self {
        GasSpecies {
            name,
            abbreviation,
            molar_mass,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GasVolume<'a> {
    // A finite amount of a particular gas
    species: &'a GasSpecies, // Reference to the type of gas
    mass: f32,        // [kg] amount of gas in the volume
    temperature: f32, // [K] temperature
    pressure: f32,    // [Pa] pressure
    volume: f32,      // [m³] volume
}

impl<'a> fmt::Display for GasVolume<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:}: {:} kg | {:} K | {:} Pa | {:} m³",
            self.species.name, self.mass, self.temperature, self.pressure, self.volume,
        )
    }
}

impl<'a> GasVolume<'a> {
    pub fn new(species: &'a GasSpecies, mass: f32) -> Self {
        // Create a new gas volume as a finite amount of mass (kg) of a
        // particular species of gas. Gas is initialized at standard
        // temperature and pressure.
        GasVolume {
            species,
            mass,
            temperature: STANDARD_TEMPERATURE,
            pressure: STANDARD_PRESSURE,
            volume: volume(
                STANDARD_TEMPERATURE,
                STANDARD_PRESSURE,
                mass,
                species.molar_mass, // Accessing molar mass through the reference
            ),
        }
    }

    pub fn update_from_ambient(&mut self, atmo: Atmosphere) {
        self.temperature = atmo.temperature();
        self.pressure = atmo.pressure();
    }

    pub fn set_temperature(&mut self, new_temperature: f32) {
        // set the temperature (K) of the GasVolume
        self.temperature = new_temperature;
    }

    pub fn set_pressure(&mut self, new_pressure: f32) {
        // set the pressure (Pa) of the GasVolume
        self.pressure = new_pressure;
    }

    pub fn set_volume(&mut self, new_volume: f32) {
        // set the volume (m³) of the GasVolume and update the pressure
        // according to the ideal gas law.
        self.volume = new_volume;
        let new_pressure = ((self.mass / self.species.molar_mass) * R * self.temperature)
            / self.volume;
        self.set_pressure(new_pressure);
    }

    pub fn set_mass(&mut self, new_mass: f32) {
        // set the mass (kg) of the GasVolume
        if new_mass >= 0.0 {
            self.mass = new_mass;
        } else {
            error!("Cannot set mass below zero!")
        }
    }

    pub fn set_mass_from_volume(&mut self) {
        // set the mass (kg) based on the current volume (m³),
        // density (kg/m³), and molar mass (kg/mol)
        self.mass = self.volume * (self.species.molar_mass / R) * (self.pressure / self.temperature);
    }

    pub fn temperature(self) -> f32 {
        // temperature (K)
        self.temperature
    }

    pub fn pressure(self) -> f32 {
        // pressure (Pa)
        self.pressure
    }

    pub fn mass(self) -> f32 {
        // mass (kg)
        self.mass
    }
    pub fn density(self) -> f32 {
        // density (kg/m³)
        density(self.temperature, self.pressure, self.species.molar_mass)
    }

    pub fn volume(&self) -> f32 {
        // volume (m³)
        volume(self.temperature, self.pressure, self.mass, self.species.molar_mass)
    }
}

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

fn volume(temperature: f32, pressure: f32, mass: f32, molar_mass: f32) -> f32 {
    // Volume (m³) of an ideal gas from its temperature (K), pressure (Pa),
    // mass (kg) and molar mass (kg/mol).
    (mass / molar_mass) * R * temperature / pressure // [m³]
}

fn density(temperature: f32, pressure: f32, molar_mass: f32) -> f32 {
    // Density (kg/m³) of an ideal gas frorm its temperature (K), pressure (Pa),
    // and molar mass (kg/mol)
    (molar_mass * pressure) / (R * temperature) // [kg/m³]
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

fn celsius2kelvin(deg_celsius: f32) -> f32 {
    // Convert degrees C to Kelvin
    deg_celsius + 273.15
}

#[derive(Debug, Deserialize)]
struct GasConfig {
    gases: Vec<GasSpecies>,
}

fn load_gas_config(file_path: &str) -> Vec<GasSpecies> {
    let content = fs::read_to_string(file_path).expect("Unable to read file");
    let config: GasConfig = from_str(&content).expect("Unable to parse RON");
    config.gases
}
