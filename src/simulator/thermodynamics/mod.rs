//! Thermodynamics
#![allow(dead_code)]
#![allow(unused_imports)]

use std::ops::{Add, Div, Mul, Sub};

mod heat;
mod ideal_gas;

pub use heat::*;
pub use ideal_gas::*;

use avian3d::{math::Scalar, prelude::Mass};
use bevy::{prelude::*, reflect::Reflect};

use crate::simulator::dynamics::Volume;

pub const BOLTZMANN_CONSTANT: f32 = 1.38e-23_f32; // [J/K]
pub const AVOGADRO_CONSTANT: f32 = 6.022e+23_f32; // [1/mol]
pub const R: f32 = BOLTZMANN_CONSTANT * AVOGADRO_CONSTANT; // [J/K-mol] Ideal gas constant

/// Plugin for thermodynamics systems
pub struct ThermodynamicsPlugin;

impl Plugin for ThermodynamicsPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_plugins();
    }
}

/// Temperature (K)
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct Temperature(pub Scalar);

impl Temperature {
    pub const STANDARD: Self = Temperature(273.15);

    pub fn new(kelvin: f32) -> Self {
        Temperature(kelvin)
    }

    pub fn from_celsius(deg_celsius: f32) -> Self {
        Temperature(deg_celsius + 273.15)
    }

    pub fn kelvin(&self) -> f32 {
        self.0
    }

    pub fn celsius(&self) -> f32 {
        self.kelvin() - 273.15
    }
}

impl Add<Temperature> for Temperature {
    type Output = Temperature;

    fn add(self, rhs: Temperature) -> Self::Output {
        Temperature(self.0 + rhs.0)
    }
}

impl Sub<Temperature> for Temperature {
    type Output = Temperature;

    fn sub(self, rhs: Temperature) -> Self::Output {
        Temperature(self.0 - rhs.0)
    }
}

impl Mul<Scalar> for Temperature {
    type Output = Temperature;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Temperature(self.0 * rhs)
    }
}

impl Div<Scalar> for Temperature {
    type Output = Temperature;

    fn div(self, rhs: Scalar) -> Self::Output {
        Temperature(self.0 / rhs)
    }
}

/// Pressure (Pa)
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct Pressure(pub Scalar);

impl Pressure {
    pub const STANDARD: Self = Pressure(101325.0);

    pub fn new(pascal: f32) -> Self {
        Pressure(pascal)
    }

    pub fn from_kilopascal(kilopascal: f32) -> Self {
        Pressure(kilopascal * 1000.0)
    }

    pub fn pascal(&self) -> f32 {
        self.0
    }

    pub fn kilopascal(&self) -> f32 {
        self.pascal() / 1000.0
    }
}

impl Add<Pressure> for Pressure {
    type Output = Pressure;

    fn add(self, rhs: Pressure) -> Self::Output {
        Pressure(self.0 + rhs.0)
    }
}

impl Sub<Pressure> for Pressure {
    type Output = Pressure;

    fn sub(self, rhs: Pressure) -> Self::Output {
        Pressure(self.0 - rhs.0)
    }
}

impl Mul<Scalar> for Pressure {
    type Output = Pressure;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Pressure(self.0 * rhs)
    }
}

impl Div<Scalar> for Pressure {
    type Output = Pressure;

    fn div(self, rhs: Scalar) -> Self::Output {
        Pressure(self.0 / rhs)
    }
}

/// Density (kg/m³)
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct Density(pub Scalar);

impl Density {
    pub const ZERO: Self = Density(0.0);

    pub fn new(kilograms: Mass, volume: Volume) -> Self {
        Density(kilograms.0 / volume.0)
    }

    pub fn kilograms_per_cubic_meter(&self) -> f32 {
        self.0
    }

    pub fn kg_per_m3(&self) -> f32 {
        self.0
    }

    pub fn from_gas(
        temperature: Temperature,
        pressure: Pressure,
        molar_mass: MolarMass,
    ) -> Density {
        Density((molar_mass.kilograms_per_mole() * pressure.pascal()) / (R * temperature.kelvin()))
    }
}

impl Add<Density> for Density {
    type Output = Density;

    fn add(self, rhs: Density) -> Self::Output {
        Density(self.0 + rhs.0)
    }
}

impl Sub<Density> for Density {
    type Output = Density;

    fn sub(self, rhs: Density) -> Self::Output {
        Density(self.0 - rhs.0)
    }
}

impl Mul<Scalar> for Density {
    type Output = Density;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Density(self.0 * rhs)
    }
}

impl Div<Scalar> for Density {
    type Output = Density;

    fn div(self, rhs: Scalar) -> Self::Output {
        Density(self.0 / rhs)
    }
}
