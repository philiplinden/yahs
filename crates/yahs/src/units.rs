use avian3d::math::Scalar;
use bevy::prelude::*;
use std::ops::{Add, Div, Mul, Sub};

/// Temperature (K)
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct TemperatureUnit(pub Scalar);

impl TemperatureUnit {
    pub const STANDARD: Self = TemperatureUnit(273.15);

    pub fn new(kelvin: f32) -> Self {
        TemperatureUnit(kelvin)
    }

    pub fn from_celsius(deg_celsius: f32) -> Self {
        TemperatureUnit(deg_celsius + 273.15)
    }

    pub fn kelvin(&self) -> f32 {
        self.0
    }

    pub fn celsius(&self) -> f32 {
        self.kelvin() - 273.15
    }

    pub fn standard(&self) -> Self {
        TemperatureUnit(273.15)
    }
}

impl Default for TemperatureUnit {
    fn default() -> Self {
        TemperatureUnit::STANDARD
    }
}

impl From<Scalar> for TemperatureUnit {
    fn from(value: Scalar) -> Self {
        TemperatureUnit(value)
    }
}

impl Into<Scalar> for TemperatureUnit {
    fn into(self) -> Scalar {
        self.0
    }
}

impl Add<TemperatureUnit> for TemperatureUnit {
    type Output = TemperatureUnit;

    fn add(self, rhs: TemperatureUnit) -> Self::Output {
        TemperatureUnit(self.0 + rhs.0)
    }
}

impl Sub<TemperatureUnit> for TemperatureUnit {
    type Output = TemperatureUnit;

    fn sub(self, rhs: TemperatureUnit) -> Self::Output {
        TemperatureUnit(self.0 - rhs.0)
    }
}

impl Mul<Scalar> for TemperatureUnit {
    type Output = TemperatureUnit;

    fn mul(self, rhs: Scalar) -> Self::Output {
        TemperatureUnit(self.0 * rhs)
    }
}

impl Div<Scalar> for TemperatureUnit {
    type Output = TemperatureUnit;

    fn div(self, rhs: Scalar) -> Self::Output {
        TemperatureUnit(self.0 / rhs)
    }
}

/// Pressure (Pa)
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct PressureUnit(pub Scalar);

impl PressureUnit {
    pub const STANDARD: Self = PressureUnit(101325.0);

    pub fn new(pascal: f32) -> Self {
        PressureUnit(pascal)
    }

    pub fn from_kilopascals(kilopascals: f32) -> Self {
        PressureUnit(kilopascals * 1000.0)
    }

    pub fn from_atmospheres(atmospheres: f32) -> Self {
        PressureUnit(atmospheres * 101325.0)
    }

    pub fn pascals(&self) -> f32 {
        self.0
    }

    pub fn kilopascals(&self) -> f32 {
        self.pascals() / 1000.0
    }

    pub fn atmospheres(&self) -> f32 {
        self.pascals() / 101325.0
    }

    pub fn standard(&self) -> Self {
        PressureUnit::from_atmospheres(1.0)
    }
}

impl Default for PressureUnit {
    fn default() -> Self {
        PressureUnit::STANDARD
    }
}

impl From<Scalar> for PressureUnit {
    fn from(value: Scalar) -> Self {
        PressureUnit(value)
    }
}

impl Into<Scalar> for PressureUnit {
    fn into(self) -> Scalar {
        self.0
    }
}

impl Add<PressureUnit> for PressureUnit {
    type Output = PressureUnit;

    fn add(self, rhs: PressureUnit) -> Self::Output {
        PressureUnit(self.0 + rhs.0)
    }
}

impl Sub<PressureUnit> for PressureUnit {
    type Output = PressureUnit;

    fn sub(self, rhs: PressureUnit) -> Self::Output {
        PressureUnit(self.0 - rhs.0)
    }
}

impl Mul<Scalar> for PressureUnit {
    type Output = PressureUnit;

    fn mul(self, rhs: Scalar) -> Self::Output {
        PressureUnit(self.0 * rhs)
    }
}

impl Div<Scalar> for PressureUnit {
    type Output = PressureUnit;

    fn div(self, rhs: Scalar) -> Self::Output {
        PressureUnit(self.0 / rhs)
    }
}

/// The volume of a body in cubic meters.
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub struct VolumeUnit(pub Scalar);

impl VolumeUnit {
    /// Zero volume.
    pub const ZERO: Self = Self(0.0);

    pub fn cubic_meters(&self) -> f32 {
        self.0
    }

    pub fn m3(&self) -> f32 {
        self.0
    }

    pub fn from_cubic_meters(cubic_meters: f32) -> Self {
        Self(cubic_meters)
    }
}

impl Add<VolumeUnit> for VolumeUnit {
    type Output = VolumeUnit;

    fn add(self, rhs: VolumeUnit) -> Self::Output {
        VolumeUnit(self.0 + rhs.0)
    }
}

impl Sub<VolumeUnit> for VolumeUnit {
    type Output = VolumeUnit;

    fn sub(self, rhs: VolumeUnit) -> Self::Output {
        VolumeUnit(self.0 - rhs.0)
    }
}

impl Mul<Scalar> for VolumeUnit {
    type Output = VolumeUnit;

    fn mul(self, rhs: Scalar) -> Self::Output {
        VolumeUnit(self.0 * rhs)
    }
}

impl Div<Scalar> for VolumeUnit {
    type Output = VolumeUnit;

    fn div(self, rhs: Scalar) -> Self::Output {
        VolumeUnit(self.0 / rhs)
    }
}

/// The mass of a body in kilograms.
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub struct MassUnit(pub Scalar);

impl MassUnit {
    /// Zero Mass.
    pub const ZERO: Self = Self(0.0);

    pub fn kilograms(&self) -> f32 {
        self.0
    }

    pub fn kg(&self) -> f32 {
        self.0
    }

    pub fn from_kilograms(kilograms: f32) -> Self {
        Self(kilograms)
    }

    pub fn from_grams(grams: f32) -> Self {
        Self(grams / 1000.0)
    }
}

impl Add<MassUnit> for MassUnit {
    type Output = MassUnit;

    fn add(self, rhs: MassUnit) -> Self::Output {
        MassUnit(self.0 + rhs.0)
    }
}

impl Sub<MassUnit> for MassUnit {
    type Output = MassUnit;

    fn sub(self, rhs: MassUnit) -> Self::Output {
        MassUnit(self.0 - rhs.0)
    }
}

impl Mul<Scalar> for MassUnit {
    type Output = MassUnit;

    fn mul(self, rhs: Scalar) -> Self::Output {
        MassUnit(self.0 * rhs)
    }
}

impl Div<Scalar> for MassUnit {
    type Output = MassUnit;

    fn div(self, rhs: Scalar) -> Self::Output {
        MassUnit(self.0 / rhs)
    }
}

/// Density (kg/m³)
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct DensityUnit(pub Scalar);

impl Default for DensityUnit {
    fn default() -> Self {
        DensityUnit::ZERO
    }
}

impl DensityUnit {
    pub const ZERO: Self = DensityUnit(0.0);

    pub fn new(kilograms: Scalar, cubic_meters: Scalar) -> Self {
        DensityUnit(kilograms / cubic_meters)
    }

    pub fn kilograms_per_cubic_meter(&self) -> f32 {
        self.0
    }

    pub fn kg_per_m3(&self) -> f32 {
        self.kilograms_per_cubic_meter()
    }
}

impl From<Scalar> for DensityUnit {
    fn from(value: Scalar) -> Self {
        DensityUnit(value)
    }
}

impl Into<Scalar> for DensityUnit {
    fn into(self) -> Scalar {
        self.kilograms_per_cubic_meter()
    }
}

impl Add<DensityUnit> for DensityUnit {
    type Output = DensityUnit;

    fn add(self, rhs: DensityUnit) -> Self::Output {
        DensityUnit(self.0 + rhs.0)
    }
}

impl Sub<DensityUnit> for DensityUnit {
    type Output = DensityUnit;

    fn sub(self, rhs: DensityUnit) -> Self::Output {
        DensityUnit(self.0 - rhs.0)
    }
}

impl Mul<Scalar> for DensityUnit {
    type Output = DensityUnit;

    fn mul(self, rhs: Scalar) -> Self::Output {
        DensityUnit(self.0 * rhs)
    }
}

impl Div<Scalar> for DensityUnit {
    type Output = DensityUnit;

    fn div(self, rhs: Scalar) -> Self::Output {
        DensityUnit(self.0 / rhs)
    }
}
