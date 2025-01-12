//! Basic physical properties.
#![allow(dead_code)]

use std::ops::{Add, Div, Mul, Sub};

use avian3d::{
    math::{Scalar, PI},
    prelude::Mass,
};
use bevy::{prelude::*, reflect::Reflect};

use crate::shape::Volume;

pub const BOLTZMANN_CONSTANT: f32 = 1.38e-23_f32; // [J/K]
pub const AVOGADRO_CONSTANT: f32 = 6.022e+23_f32; // [1/mol]

pub const STANDARD_G: f32 = 9.80665; // [m/s^2] standard gravitational acceleration
pub const EARTH_RADIUS_M: f32 = 6371007.2; // [m] mean radius of Earth

pub struct ThermodynamicsPlugin;

impl Plugin for ThermodynamicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Temperature>();
        app.register_type::<Pressure>();
        app.register_type::<Density>();
        app.add_systems(FixedUpdate, update_density);
    }
}

/// Temperature (K)
#[derive(Component,Debug, Clone, Copy, PartialEq, Reflect)]
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

impl Default for Temperature {
    fn default() -> Self {
        Temperature::STANDARD
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
#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect)]
pub struct Pressure(pub Scalar);

impl Pressure {
    pub const STANDARD: Self = Pressure(101325.0);

    pub fn new(pascal: f32) -> Self {
        Pressure(pascal)
    }

    pub fn from_kilopascals(kilopascals: f32) -> Self {
        Pressure(kilopascals * 1000.0)
    }

    pub fn pascals(&self) -> f32 {
        self.0
    }

    pub fn kilopascals(&self) -> f32 {
        self.pascals() / 1000.0
    }
}

impl Default for Pressure {
    fn default() -> Self {
        Pressure::STANDARD
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
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub struct Density(pub Scalar);

impl Density {
    pub const ZERO: Self = Density(0.0);

    pub fn new(kilograms: Mass, volume: Volume) -> Self {
        Density(kilograms.0 / volume.cubic_meters())
    }

    pub fn kilograms_per_cubic_meter(&self) -> f32 {
        self.0
    }

    pub fn kg_per_m3(&self) -> f32 {
        self.0
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

fn update_density(mut query: Query<(&mut Density, &Mass, &Volume)>) {
    for (mut density, mass, volume) in query.iter_mut() {
        density.0 = mass.0 / volume.0;
    }
}
