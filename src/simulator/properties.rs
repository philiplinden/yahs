//! Basic physical properties.

#![allow(dead_code)]

use std::ops::{Add, Div, Mul, Sub};

use avian3d::{
    math::{Scalar, PI},
    prelude::{ColliderDensity, ColliderMassProperties, PhysicsSet, RigidBody},
};
use bevy::{prelude::*, reflect::Reflect};
#[cfg(feature = "config-files")]
use serde::{Deserialize, Serialize};

pub const BOLTZMANN_CONSTANT: f32 = 1.38e-23_f32; // [J/K]
pub const AVOGADRO_CONSTANT: f32 = 6.022e+23_f32; // [1/mol]

pub const STANDARD_G: f32 = 9.80665; // [m/s^2] standard gravitational acceleration
pub const EARTH_RADIUS_M: f32 = 6371007.2; // [m] mean radius of Earth

fn sphere_volume(radius: f32) -> f32 {
    (4.0 / 3.0) * PI * f32::powf(radius, 3.0)
}

fn shell_volume(internal_radius: f32, thickness: f32) -> f32 {
    let external_radius = internal_radius + thickness;
    let internal_volume = sphere_volume(internal_radius);
    let external_volume = sphere_volume(external_radius);
    external_volume - internal_volume
}

fn sphere_radius_from_volume(volume: f32) -> f32 {
    f32::powf(volume, 1.0 / 3.0) / (4.0 / 3.0) * PI
}

fn sphere_surface_area(radius: f32) -> f32 {
    4.0 * PI * f32::powf(radius, 2.0)
}

pub struct CorePropertiesPlugin;

impl Plugin for CorePropertiesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Temperature>();
        app.register_type::<Pressure>();
        app.register_type::<Volume>();
        app.register_type::<Density>();
        app.register_type::<Mass>();
        app.register_type::<MolarMass>();

        // Ensure that the Avian density matches our computed mass and density
        // before it starts solving physics.
        app.add_systems(
            Update,
            (sync_avian_mass, sync_avian_density)
                .chain()
                .in_set(PhysicsSet::Prepare),
        );
    }
}

/// Temperature (K)
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Reflect)]
#[cfg_attr(feature = "config-files", derive(Serialize, Deserialize))]
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
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Reflect)]
#[cfg_attr(feature = "config-files", derive(Serialize, Deserialize))]
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

/// The volume of a body in cubic meters.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Reflect)]
#[cfg_attr(feature = "config-files", derive(Serialize, Deserialize))]
pub struct Volume(pub Scalar);

impl Volume {
    /// Zero volume.
    pub const ZERO: Self = Self(0.0);

    pub fn cubic_meters(&self) -> f32 {
        self.0
    }

    pub fn m3(&self) -> f32 {
        self.0
    }
}

impl Add<Volume> for Volume {
    type Output = Volume;

    fn add(self, rhs: Volume) -> Self::Output {
        Volume(self.0 + rhs.0)
    }
}

impl Sub<Volume> for Volume {
    type Output = Volume;

    fn sub(self, rhs: Volume) -> Self::Output {
        Volume(self.0 - rhs.0)
    }
}

impl Mul<Scalar> for Volume {
    type Output = Volume;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Volume(self.0 * rhs)
    }
}

impl Div<Scalar> for Volume {
    type Output = Volume;

    fn div(self, rhs: Scalar) -> Self::Output {
        Volume(self.0 / rhs)
    }
}

/// Density (kg/m³)
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Reflect)]
#[cfg_attr(feature = "config-files", derive(Serialize, Deserialize))]
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

fn sync_avian_density(mut densities: Query<(&mut ColliderDensity, &Volume, &Mass)>) {
    for (mut density, volume, mass) in densities.iter_mut() {
        let our_density = mass.kg() / volume.m3();
        density.0 = our_density;
    }
}

/// Mass (kg)
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Reflect)]
#[cfg_attr(feature = "config-files", derive(Serialize, Deserialize))]
pub struct Mass(pub Scalar);

impl Mass {
    pub fn kilograms(&self) -> f32 {
        self.0
    }

    pub fn kg(&self) -> f32 {
        self.0
    }

    pub fn from_mass_properties(mass_props: ColliderMassProperties) -> Self {
        Mass(mass_props.mass.0)
    }
}

impl Add<Mass> for Mass {
    type Output = Mass;

    fn add(self, rhs: Mass) -> Self::Output {
        Mass(self.0 + rhs.0)
    }
}

impl Sub<Mass> for Mass {
    type Output = Mass;

    fn sub(self, rhs: Mass) -> Self::Output {
        Mass(self.0 - rhs.0)
    }
}

impl Mul<Scalar> for Mass {
    type Output = Mass;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Mass(self.0 * rhs)
    }
}

impl Div<Volume> for Mass {
    type Output = Density;

    fn div(self, rhs: Volume) -> Self::Output {
        Density(self.0 / rhs.0)
    }
}

impl Div<Scalar> for Mass {
    type Output = Mass;

    fn div(self, rhs: Scalar) -> Self::Output {
        Mass(self.0 / rhs)
    }
}

fn sync_avian_mass(mut bodies: Query<(&mut ColliderMassProperties, &Mass)>) {
    for (mut mass_props, mass) in bodies.iter_mut() {
        mass_props.mass.0 = mass.0;
    }
}

/// Molar mass (kg/mol) of a substance.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Reflect)]
#[cfg_attr(feature = "config-files", derive(Serialize, Deserialize))]
pub struct MolarMass(pub Scalar);

impl MolarMass {
    pub fn kilograms_per_mole(&self) -> f32 {
        self.0
    }
}

impl Mul<Scalar> for MolarMass {
    type Output = MolarMass;

    fn mul(self, rhs: Scalar) -> Self::Output {
        MolarMass(self.0 * rhs)
    }
}

impl Div<Scalar> for MolarMass {
    type Output = MolarMass;

    fn div(self, rhs: Scalar) -> Self::Output {
        MolarMass(self.0 / rhs)
    }
}
