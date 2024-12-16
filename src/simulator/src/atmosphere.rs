//! Atmosphere model based on the US Standard Atmosphere, 1976.
//!
//! Reference:
//! - https://apps.dtic.mil/dtic/tr/fulltext/u2/a035728.pdf
//! - https://www.translatorscafe.com/unit-converter/en-US/calculator/altitude
//! - https://www.grc.nasa.gov/WWW/K-12/airplane/atmosmet.html

use avian3d::prelude::Position;
use bevy::prelude::*;

use crate::{
    ideal_gas::{ideal_gas_density, GasSpecies},
    properties::{Density, Pressure, Temperature},
    core::{SimulationUpdateOrder, SimState},
    balloon::Balloon,
};

pub struct AtmospherePlugin;
impl Plugin for AtmospherePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Atmosphere);
        app.add_systems(
            Update,
            pause_on_out_of_bounds.in_set(SimulationUpdateOrder::First),
        );
    }
}

fn pause_on_out_of_bounds(
    positions: Query<&Position, With<Balloon>>,
    mut state: ResMut<NextState<SimState>>,
) {
    for position in positions.iter() {
        if position.y < Atmosphere::MIN_ALTITUDE || position.y > Atmosphere::MAX_ALTITUDE {
            error!("Atmosphere out of bounds: {}", position.y);
            state.set(SimState::Stopped);
        }
    }
}

/// US Standard Atmosphere, 1976
#[derive(Resource)]
pub struct Atmosphere;

impl Atmosphere {
    pub const MAX_ALTITUDE: f32 = 84999.0; // small margin to avoid panics
    pub const MIN_ALTITUDE: f32 = -56.0; // small margin to avoid panics

    /// Temperature (K) of the atmosphere at a position.
    pub fn temperature(&self, position: Vec3) -> Temperature {
        // TODO: Look up temperature based on latitude, longitude, not just altitude
        coesa_temperature(position.y).unwrap_or_else(|e| {
            error!("Atmosphere temperature out of bounds: {}", e);
            Temperature::STANDARD
        }) // we should handle this better
    }

    /// Pressure (Pa) of the atmosphere at a position.
    pub fn pressure(&self, position: Vec3) -> Pressure {
        // TODO: Look up pressure based on latitude, longitude, not just altitude
        coesa_pressure(position.y).unwrap_or_else(|e| {
            error!("Atmosphere pressure out of bounds: {}", e);
            Pressure::STANDARD
        }) // we should handle this better
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

#[derive(Debug)]
enum AtmosphereError {
    // v Fancy stuff from thiserror crate
    // #[error(
    //     "Altitude {0} m is outside of the accepted range! Must be {min}-{max} m",
    //     min = Atmosphere::MIN_ALTITUDE,
    //     max = Atmosphere::MAX_ALTITUDE
    // )]
    // OutOfBounds(f32),
    #[allow(dead_code)]
    OutOfBounds(f32),
}

impl std::fmt::Display for AtmosphereError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Temperature (K) of the atmosphere at a given altitude (m).
/// Only valid for altitudes below 85,000 meters.
/// Based on the US Standard Atmosphere, 1976. (aka COESA)
fn coesa_temperature(altitude: f32) -> Result<Temperature, AtmosphereError> {
    if (-57.0..11000.0).contains(&altitude) {
        Ok(Temperature::from_celsius(15.04 - 0.00649 * altitude))
    } else if (11000.0..25000.0).contains(&altitude) {
        Ok(Temperature::from_celsius(-56.46))
    } else if (25000.0..85000.0).contains(&altitude) {
        Ok(Temperature::from_celsius(-131.21 + 0.00299 * altitude))
    } else {
        Err(AtmosphereError::OutOfBounds(altitude))
    }
}

/// Pressure (Pa) of the atmosphere at a given altitude (m).
/// Only valid for altitudes below 85,000 meters.
/// Based on the US Standard Atmosphere, 1976. (aka COESA)
fn coesa_pressure(altitude: f32) -> Result<Pressure, AtmosphereError> {
    if (-57.0..11000.0).contains(&altitude) {
        Ok(Pressure::from_kilopascals(
            101.29
                * f32::powf(
                    coesa_temperature(altitude).unwrap_or_default().kelvin() / 288.08,
                    5.256,
                ),
        ))
    } else if (11000.0..25000.0).contains(&altitude) {
        Ok(Pressure::from_kilopascals(
            22.65 * f32::exp(1.73 - 0.000157 * altitude),
        ))
    } else if (25000.0..85000.0).contains(&altitude) {
        Ok(Pressure::from_kilopascals(
            2.488
                * f32::powf(
                    coesa_temperature(altitude).unwrap_or_default().kelvin() / 216.6,
                    -11.388,
                ),
        ))
    } else {
        Err(AtmosphereError::OutOfBounds(altitude))
    }
}
