#![allow(dead_code)]
//! Pre-computed constants in SI units.
//! 
//! The purpose of this module is for convenience and neatness. Rather than
//! requiring other modules to find and instantiate the correct unuts for a
//! given constant, we can just use these.
//! 
//! All constants are computed using the `uom` crate and support conversion.
//! 
//! TODO: Add an option to support f64 features that are available in the crates
//! that `buoy-core` depends on.

use std::sync::LazyLock;
use uom::si::{
    acceleration::standard_gravity, f32::*, heat_capacity::boltzmann_constant, length::meter,
    molar_heat_capacity::molar_gas_constant, pressure::pascal, thermodynamic_temperature::kelvin,
};

pub static PI: f32 = std::f32::consts::PI;
pub static BOLTZMANN_CONSTANT: LazyLock<HeatCapacity> =
    LazyLock::new(|| HeatCapacity::new::<boltzmann_constant>(1.0));
pub static GAS_CONSTANT: LazyLock<MolarHeatCapacity> =
    LazyLock::new(|| MolarHeatCapacity::new::<molar_gas_constant>(1.0));

pub static STANDARD_GRAVITY: LazyLock<Acceleration> =
    LazyLock::new(|| Acceleration::new::<standard_gravity>(1.0));
pub static STANDARD_TEMPERATURE: LazyLock<ThermodynamicTemperature> =
    LazyLock::new(|| ThermodynamicTemperature::new::<kelvin>(273.15));
pub static STANDARD_PRESSURE: LazyLock<Pressure> =
    LazyLock::new(|| Pressure::new::<pascal>(101325.0));

pub static EARTH_RADIUS_M: LazyLock<Length> = LazyLock::new(|| Length::new::<meter>(6371007.2));
