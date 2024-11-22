//! Atmosphere model based on the US Standard Atmosphere, 1976.
//!
//! Reference:
//! - https://apps.dtic.mil/dtic/tr/fulltext/u2/a035728.pdf
//! - https://www.translatorscafe.com/unit-converter/en-US/calculator/altitude
//! - https://www.grc.nasa.gov/WWW/K-12/airplane/atmosmet.html

use bevy::prelude::*;

use super::{
    ideal_gas::{ideal_gas_density, GasSpecies},
    Density, Position, Pressure, SimState, SimulatedBody, Temperature,
};

pub struct AtmospherePlugin;
impl Plugin for AtmospherePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Atmosphere);
        app.add_systems(Update, fault_if_out_of_bounds);
    }
}

/// US Standard Atmosphere, 1976
#[derive(Resource)]
pub struct Atmosphere;

impl Atmosphere {
    pub const MAX_ALTITUDE: f32 = 84999.0; // small margin to avoid panics
    pub const MIN_ALTITUDE: f32 = -56.0; // small margin to avoid panics

    pub fn out_of_bounds(&self, position: Vec3) -> bool {
        match position.y {
            y if (y > Atmosphere::MAX_ALTITUDE) => true,
            y if (y < Atmosphere::MIN_ALTITUDE) => true,
            _ => false,
        }
    }

    /// Temperature (K) of the atmosphere at a position.
    pub fn temperature(&self, position: Vec3) -> Temperature {
        // TODO: Look up temperature based on latitude, longitude, not just altitude
        coesa_temperature(position.y).unwrap() // we should handle this better
    }

    /// Pressure (Pa) of the atmosphere at a position.
    pub fn pressure(&self, position: Vec3) -> Pressure {
        // TODO: Look up pressure based on latitude, longitude, not just altitude
        coesa_pressure(position.y).unwrap() // we should handle this better
    }

    /// Density (kg/m³) of the atmosphere at a position.
    pub fn density(&self, position: Vec3) -> Density {
        ideal_gas_density(
            self.temperature(position),
            self.pressure(position),
            &GasSpecies::air(),
        )
    }
}

/// If any of the simulated bodies are out of bounds, set the app state to anomaly
/// TODO: we should use an event for this
fn fault_if_out_of_bounds(
    atmosphere: Res<Atmosphere>,
    bodies: Query<(Entity, &Position), With<SimulatedBody>>,
    mut next_state: ResMut<NextState<SimState>>,
) {
    for (_, position) in bodies.iter() {
        if atmosphere.out_of_bounds(position.0) {
            next_state.set(SimState::Anomaly)
        };
    }
}

/// Temperature (K) of the atmosphere at a given altitude (m).
/// Only valid for altitudes below 85,000 meters.
/// Based on the US Standard Atmosphere, 1976. (aka COESA)
fn coesa_temperature(altitude: f32) -> Result<Temperature, String> {
    if (-57.0..11000.0).contains(&altitude) {
        Ok(Temperature::from_celsius(15.04 - 0.00649 * altitude))
    } else if (11000.0..25000.0).contains(&altitude) {
        Ok(Temperature::from_celsius(-56.46))
    } else if (25000.0..85000.0).contains(&altitude) {
        Ok(Temperature::from_celsius(-131.21 + 0.00299 * altitude))
    } else {
        Err(format!(
            "Altitude {:}m is outside of the accepted range! Must be 0-85000m",
            altitude
        ))
    }
}

/// Pressure (Pa) of the atmosphere at a given altitude (m).
/// Only valid for altitudes below 85,000 meters.
/// Based on the US Standard Atmosphere, 1976. (aka COESA)
fn coesa_pressure(altitude: f32) -> Result<Pressure, String> {
    if (-57.0..11000.0).contains(&altitude) {
        Ok(Pressure::from_kilopascal(
            101.29
                * f32::powf(
                    coesa_temperature(altitude).unwrap_or_default().kelvin() / 288.08,
                    5.256,
                ),
        ))
    } else if (11000.0..25000.0).contains(&altitude) {
        Ok(Pressure::from_kilopascal(
            22.65 * f32::exp(1.73 - 0.000157 * altitude),
        ))
    } else if (25000.0..85000.0).contains(&altitude) {
        Ok(Pressure::from_kilopascal(
            2.488
                * f32::powf(
                    coesa_temperature(altitude).unwrap_or_default().kelvin() / 216.6,
                    -11.388,
                ),
        ))
    } else {
        Err(format!(
            "Altitude {:}m is outside of the accepted range! Must be 0-85000m",
            altitude
        ))
    }
}
