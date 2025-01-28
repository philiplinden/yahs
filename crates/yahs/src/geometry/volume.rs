use std::ops::{Add, Div, Mul, Sub};

use bevy::prelude::*;
use avian3d::math::{Scalar, PI};

pub(crate) fn sphere_volume(radius: f32) -> f32 {
    (4.0 / 3.0) * PI * f32::powf(radius, 3.0)
}

pub(crate) fn sphere_radius_from_volume(volume: f32) -> f32 {
    f32::powf(volume * 3.0 / (4.0 * PI), 1.0 / 3.0)
}

#[allow(dead_code)]
pub(crate) fn shell_volume(internal_radius: f32, thickness: f32) -> f32 {
    let external_radius = internal_radius + thickness;
    let internal_volume = sphere_volume(internal_radius);
    let external_volume = sphere_volume(external_radius);
    external_volume - internal_volume
}

/// The volume of a body in cubic meters.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Reflect)]
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

    pub fn from_cubic_meters(cubic_meters: f32) -> Self {
        Self(cubic_meters)
    }

    pub fn sphere(radius: f32) -> Self {
        Self(sphere_volume(radius))
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
