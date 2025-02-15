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

    pub fn pa(&self) -> f32 {
        self.pascals()
    }

    pub fn kilopascals(&self) -> f32 {
        self.pascals() / 1000.0
    }

    pub fn kpa(&self) -> f32 {
        self.kilopascals()
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

impl Div<VolumeUnit> for VolumeUnit {
    type Output = Scalar;

    fn div(self, rhs: VolumeUnit) -> Self::Output {
        self.0 / rhs.0
    }
}

impl Div<DistanceUnit> for VolumeUnit {
    type Output = AreaUnit;

    fn div(self, rhs: DistanceUnit) -> Self::Output {
        AreaUnit(self.0 / rhs.0)
    }
}

impl Div<AreaUnit> for VolumeUnit {
    type Output = DistanceUnit;

    fn div(self, rhs: AreaUnit) -> Self::Output {
        DistanceUnit(self.0 / rhs.0)
    }
}

/// The area of a body in square meters.
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub struct AreaUnit(pub Scalar);

impl AreaUnit {
    pub const ZERO: Self = Self(0.0);

    pub fn square_meters(&self) -> f32 {
        self.0
    }

    pub fn m2(&self) -> f32 {
        self.0
    }

    pub fn from_square_meters(square_meters: f32) -> Self {
        Self(square_meters)
    }
}

impl Mul<DistanceUnit> for AreaUnit {
    type Output = VolumeUnit;

    fn mul(self, rhs: DistanceUnit) -> Self::Output {
        VolumeUnit(self.0 * rhs.0)
    }
}

impl Div<DistanceUnit> for AreaUnit {
    type Output = DistanceUnit;

    fn div(self, rhs: DistanceUnit) -> Self::Output {
        DistanceUnit(self.0 / rhs.0)
    }
}

/// Distance unit (m)
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct DistanceUnit(pub Scalar);

impl DistanceUnit {
    pub const ZERO: Self = Self(0.0);

    pub fn new(meters: Scalar) -> Self {
        Self(meters)
    }

    pub fn millimeters(&self) -> f32 {
        self.0 * 1000.0
    }

    pub fn mm(&self) -> f32 {
        self.millimeters()
    }

    pub fn centimeters(&self) -> f32 {
        self.0 * 100.0
    }

    pub fn cm(&self) -> f32 {
        self.centimeters()
    }

    pub fn meters(&self) -> f32 {
        self.0
    }

    pub fn m(&self) -> f32 {
        self.meters()
    }

    pub fn kilometers(&self) -> f32 {
        self.0 / 1000.0
    }

    pub fn km(&self) -> f32 {
        self.kilometers()
    }

    pub fn from_millimeters(millimeters: f32) -> Self {
        Self(millimeters / 1000.0)
    }

    pub fn from_centimeters(centimeters: f32) -> Self {
        Self(centimeters / 100.0)
    }

    pub fn from_meters(meters: f32) -> Self {
        Self(meters)
    }

    pub fn from_kilometers(kilometers: f32) -> Self {
        Self(kilometers * 1000.0)
    }
}

impl Default for DistanceUnit {
    fn default() -> Self {
        DistanceUnit::ZERO
    }
}

impl From<Scalar> for DistanceUnit {
    fn from(meters: Scalar) -> Self {
        DistanceUnit(meters)
    }
}

impl Into<Scalar> for DistanceUnit {
    fn into(self) -> Scalar {
        self.0
    }
}

impl Add<DistanceUnit> for DistanceUnit {
    type Output = DistanceUnit;

    fn add(self, rhs: DistanceUnit) -> Self::Output {
        DistanceUnit(self.0 + rhs.0)
    }
}

impl Sub<DistanceUnit> for DistanceUnit {
    type Output = DistanceUnit;

    fn sub(self, rhs: DistanceUnit) -> Self::Output {
        DistanceUnit(self.0 - rhs.0)
    }
}

impl Mul<Scalar> for DistanceUnit {
    type Output = DistanceUnit;

    fn mul(self, rhs: Scalar) -> Self::Output {
        DistanceUnit(self.0 * rhs)
    }
}

impl Mul<DistanceUnit> for DistanceUnit {
    type Output = AreaUnit;

    fn mul(self, rhs: DistanceUnit) -> Self::Output {
        AreaUnit(self.0 * rhs.0)
    }
}

impl Mul<AreaUnit> for DistanceUnit {
    type Output = VolumeUnit;

    fn mul(self, rhs: AreaUnit) -> Self::Output {
        VolumeUnit(self.0 * rhs.0)
    }
}

impl Div<Scalar> for DistanceUnit {
    type Output = Scalar;

    fn div(self, rhs: Scalar) -> Self::Output {
        self.0 / rhs
    }
}

impl Div<DistanceUnit> for DistanceUnit {
    type Output = Scalar;

    fn div(self, rhs: DistanceUnit) -> Self::Output {
        self.0 / rhs.0
    }
}

impl Div<AreaUnit> for DistanceUnit {
    type Output = DistanceUnit;

    fn div(self, rhs: AreaUnit) -> Self::Output {
        DistanceUnit(self.0 / rhs.0)
    }
}

impl Div<VolumeUnit> for DistanceUnit {
    type Output = AreaUnit;

    fn div(self, rhs: VolumeUnit) -> Self::Output {
        AreaUnit(self.0 / rhs.0)
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

impl Div<VolumeUnit> for MassUnit {
    type Output = DensityUnit;

    fn div(self, rhs: VolumeUnit) -> Self::Output {
        DensityUnit(self.0 / rhs.0)
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

pub struct SpeedUnit(pub Scalar);

impl SpeedUnit {
    pub const ZERO: Self = Self(0.0);

    pub fn new(meters_per_second: Scalar) -> Self {
        Self(meters_per_second)
    }
}

impl From<Scalar> for SpeedUnit {
    fn from(value: Scalar) -> Self {
        Self(value)
    }
}

impl Into<Scalar> for SpeedUnit {
    fn into(self) -> Scalar {
        self.0
    }
}

impl Add<SpeedUnit> for SpeedUnit {
    type Output = SpeedUnit;

    fn add(self, rhs: SpeedUnit) -> Self::Output {
        SpeedUnit(self.0 + rhs.0)
    }
}

impl Sub<SpeedUnit> for SpeedUnit {
    type Output = SpeedUnit;

    fn sub(self, rhs: SpeedUnit) -> Self::Output {
        SpeedUnit(self.0 - rhs.0)
    }
}

impl Mul<Scalar> for SpeedUnit {
    type Output = SpeedUnit;

    fn mul(self, rhs: Scalar) -> Self::Output {
        SpeedUnit(self.0 * rhs)
    }
}

impl Mul<TimeUnit> for SpeedUnit {
    type Output = DistanceUnit;

    fn mul(self, rhs: TimeUnit) -> Self::Output {
        DistanceUnit(self.0 * rhs.0)
    }
}

impl Div<Scalar> for SpeedUnit {
    type Output = SpeedUnit;
    
    fn div(self, rhs: Scalar) -> Self::Output {
        SpeedUnit(self.0 / rhs)
    }
}

impl Div<TimeUnit> for SpeedUnit {
    type Output = AccelerationUnit;

    fn div(self, rhs: TimeUnit) -> Self::Output {
        AccelerationUnit(self.0 / rhs.0)
    }
}

impl Div<DistanceUnit> for SpeedUnit {
    type Output = FrequencyUnit;

    fn div(self, rhs: DistanceUnit) -> Self::Output {
        FrequencyUnit(self.0 / rhs.0)
    }
}

pub struct AccelerationUnit(pub Scalar);

impl AccelerationUnit {
    pub const ZERO: Self = Self(0.0);

    pub fn new(meters_per_second_squared: Scalar) -> Self {
        Self(meters_per_second_squared)
    }

    pub fn meters_per_second_squared(&self) -> f32 {
        self.0
    }

    pub fn m_per_s2(&self) -> f32 {
        self.meters_per_second_squared()
    }

    pub fn from_meters_per_second_squared(meters_per_second_squared: f32) -> Self {
        Self(meters_per_second_squared)
    }
}

impl Add<AccelerationUnit> for AccelerationUnit {
    type Output = AccelerationUnit;

    fn add(self, rhs: AccelerationUnit) -> Self::Output {
        AccelerationUnit(self.0 + rhs.0)
    }
}

impl Sub<AccelerationUnit> for AccelerationUnit {
    type Output = AccelerationUnit;

    fn sub(self, rhs: AccelerationUnit) -> Self::Output {
        AccelerationUnit(self.0 - rhs.0)
    }
}

impl Mul<Scalar> for AccelerationUnit {
    type Output = AccelerationUnit;

    fn mul(self, rhs: Scalar) -> Self::Output {
        AccelerationUnit(self.0 * rhs)
    }
}

impl Div<Scalar> for AccelerationUnit {
    type Output = AccelerationUnit;

    fn div(self, rhs: Scalar) -> Self::Output {
        AccelerationUnit(self.0 / rhs)
    }
}

impl Mul<TimeUnit> for AccelerationUnit {
    type Output = SpeedUnit;

    fn mul(self, rhs: TimeUnit) -> Self::Output {
        SpeedUnit(self.0 * rhs.0)
    }
}

pub struct TimeUnit(pub Scalar);

impl TimeUnit {
    pub const ZERO: Self = Self(0.0);

    pub fn new(seconds: Scalar) -> Self {
        Self(seconds)
    }

    pub fn inverse(&self) -> FrequencyUnit {
        FrequencyUnit(1.0 / self.0)
    }

    pub fn to_frequency(&self) -> FrequencyUnit {
        self.inverse()
    }

    pub fn seconds(&self) -> f32 {
        self.0
    }

    pub fn s(&self) -> f32 {
        self.seconds()
    }

    pub fn from_seconds(seconds: f32) -> Self {
        Self(seconds)
    }

    pub fn minutes(&self) -> f32 {
        self.0 / 60.0
    }

    pub fn min(&self) -> f32 {
        self.minutes()
    }

    pub fn from_minutes(minutes: f32) -> Self {
        Self(minutes * 60.0)
    }

    pub fn milliseconds(&self) -> f32 {
        self.0 * 1000.0
    }

    pub fn ms(&self) -> f32 {
        self.milliseconds()
    }

    pub fn from_milliseconds(milliseconds: f32) -> Self {
        Self(milliseconds / 1000.0)
    }

    pub fn hours(&self) -> f32 {
        self.0 / 3600.0
    }

    pub fn h(&self) -> f32 {
        self.hours()
    }

    pub fn from_hours(hours: f32) -> Self {
        Self(hours * 3600.0)
    }
}

pub struct ForceUnit(pub Scalar);

impl ForceUnit {
    pub const ZERO: Self = Self(0.0);
    
    pub fn new(newtons: Scalar) -> Self {
        Self(newtons)
    }

    pub fn newtons(&self) -> f32 {
        self.0
    }

    pub fn from_newtons(newtons: f32) -> Self {
        Self(newtons)
    }
}

impl Add<ForceUnit> for ForceUnit {
    type Output = ForceUnit;

    fn add(self, rhs: ForceUnit) -> Self::Output {
        ForceUnit(self.0 + rhs.0)
    }
}

impl Sub<ForceUnit> for ForceUnit {
    type Output = ForceUnit;

    fn sub(self, rhs: ForceUnit) -> Self::Output {
        ForceUnit(self.0 - rhs.0)
    }
}

impl Mul<Scalar> for ForceUnit {
    type Output = ForceUnit;

    fn mul(self, rhs: Scalar) -> Self::Output {
        ForceUnit(self.0 * rhs)
    }
}

impl Div<Scalar> for ForceUnit {
    type Output = ForceUnit;

    fn div(self, rhs: Scalar) -> Self::Output {
        ForceUnit(self.0 / rhs)
    }
}

impl Div<AccelerationUnit> for ForceUnit {
    type Output = MassUnit;

    fn div(self, rhs: AccelerationUnit) -> Self::Output {
        MassUnit(self.0 / rhs.0)
    }
}

impl Div<MassUnit> for ForceUnit {
    type Output = AccelerationUnit;

    fn div(self, rhs: MassUnit) -> Self::Output {
        AccelerationUnit(self.0 / rhs.0)
    }
}

pub struct FrequencyUnit(pub Scalar);

impl FrequencyUnit {
    pub const ZERO: Self = Self(0.0);
    
    pub fn new(hertz: Scalar) -> Self {
        Self(hertz)
    }

    pub fn inverse(&self) -> TimeUnit {
        TimeUnit(1.0 / self.0)
    }

    pub fn to_time(&self) -> TimeUnit {
        self.inverse()
    }

    pub fn hertz(&self) -> f32 {
        self.0
    }

    pub fn hz(&self) -> f32 {
        self.hertz()
    }

    pub fn from_hertz(hertz: f32) -> Self {
        Self(hertz)
    }

    pub fn from_seconds(seconds: f32) -> Self {
        Self(1.0 / seconds)
    }
}

impl Add<FrequencyUnit> for FrequencyUnit {
    type Output = FrequencyUnit;

    fn add(self, rhs: FrequencyUnit) -> Self::Output {
        FrequencyUnit(self.0 + rhs.0)
    }
}

impl Sub<FrequencyUnit> for FrequencyUnit {
    type Output = FrequencyUnit;

    fn sub(self, rhs: FrequencyUnit) -> Self::Output {
        FrequencyUnit(self.0 - rhs.0)
    }
}

impl Mul<Scalar> for FrequencyUnit {
    type Output = FrequencyUnit;

    fn mul(self, rhs: Scalar) -> Self::Output {
        FrequencyUnit(self.0 * rhs)
    }
}

impl Div<Scalar> for FrequencyUnit {
    type Output = FrequencyUnit;

    fn div(self, rhs: Scalar) -> Self::Output {
        FrequencyUnit(self.0 / rhs)
    }
}

impl Mul<TimeUnit> for FrequencyUnit {
    type Output = Scalar;

    fn mul(self, rhs: TimeUnit) -> Self::Output {
        self.0 * rhs.0
    }
}
