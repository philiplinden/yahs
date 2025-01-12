use std::ops::{Add, Div, Mul, Sub};

use bevy::{
    math::primitives::{Capsule3d, Cone, Cuboid, Cylinder, Sphere},
    prelude::*,
};
use avian3d::math::{Scalar, PI};

fn sphere_volume(radius: f32) -> f32 {
    (4.0 / 3.0) * PI * f32::powf(radius, 3.0)
}

fn shell_volume(internal_radius: f32, thickness: f32) -> f32 {
    let external_radius = internal_radius + thickness;
    let internal_volume = sphere_volume(internal_radius);
    let external_volume = sphere_volume(external_radius);
    external_volume - internal_volume
}

pub fn sphere_radius_from_volume(volume: f32) -> f32 {
    f32::powf(volume, 1.0 / 3.0) / (4.0 / 3.0) * PI
}

pub struct ShapeToolsPlugin;

impl Plugin for ShapeToolsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Volume>();
    }
}

/// A wrapper type for geometric primitives.
/// 
/// # Examples
/// ```
/// use bevy::math::primitives::Sphere;
/// 
/// // Create a sphere with radius 2.0
/// let sphere = Sphere::new(2.0);
/// 
/// // Convert it to PrimitiveShape using From/Into
/// let shape1 = PrimitiveShape::from(sphere);
/// let shape2: PrimitiveShape<Sphere> = sphere.into();
/// 
/// // Use PrimitiveShape methods
/// println!("Volume: {}", shape1.volume());
/// println!("Surface area: {}", shape1.area());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveShape<T: Measured3d> {
    pub shape: T,
}

impl<T: Measured3d> PrimitiveShape<T>
where
    T: Measured3d,
{
    pub fn volume(&self) -> f32 {
        self.shape.volume()
    }

    pub fn surface_area(&self) -> f32 {
        self.shape.area()
    }
}

impl PrimitiveShape<Sphere> {
    pub fn set_volume(&mut self, volume: f32) {
        self.shape.radius = sphere_radius_from_volume(volume);
    }
}

impl PrimitiveShape<Capsule3d> {
    pub fn set_volume(&mut self, _volume: f32) {
        todo!("Implement volume adjustment for capsule");
    }
}

impl<T: Measured3d> From<T> for PrimitiveShape<T> {
    fn from(value: T) -> Self {
        PrimitiveShape { shape: value }
    }
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
