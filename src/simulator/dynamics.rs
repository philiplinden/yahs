//! Forces that act in the vertical axis. All forces assume a positive-up
//! coordinate frame and are reported in Newtons.
#![allow(dead_code)]

use std::ops::{Add, Div, Mul, Sub};

use avian3d::{math::Scalar, prelude::*};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::simulator::thermodynamics::Density;

pub const STANDARD_G: f32 = 9.80665; // [m/s^2] standard gravitational acceleration
pub const EARTH_RADIUS_M: f32 = 6371007.2; // [m] mean radius of Earth

pub struct DynamicsPlugin;

impl Plugin for DynamicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VolumetricBodyPlugin);
        // app.add_systems(Update, step);
    }
}

/// Acceleration (m/s²) from gravity at an altitude (m) above mean sea level.
fn g(position: Vec3) -> Vec3 {
    Vec3::NEG_Y * (STANDARD_G * (EARTH_RADIUS_M / (EARTH_RADIUS_M + position.y)))
}

/// Weight (N) as a function of altitude (m) and mass (kg).
pub fn weight(position: Vec3, mass: f32) -> Vec3 {
    g(position) * mass // [N]
}

/// Force (N) due to air displaced by the given gas volume.
pub fn buoyancy(body: VolumetricBody, position: Vec3, ambient_density: Density) -> Vec3 {
    let v = body.volume.0;
    if v <= 0.0 {
        // No buoyancy if the volume is zero
        return Vec3::ZERO
    }

    let rho_lift = body.density().0;
    let rho_atmo = ambient_density.0;
    (v * (rho_lift - rho_atmo)) * g(position)
}

/// Force (N) due to drag as a solid body moves through a fluid.
pub fn drag(ambient_density: f32, velocity: Vec3, drag_area: f32, drag_coeff: f32) -> Vec3 {
    let direction = -velocity.normalize();
    direction * drag_coeff / 2.0 * ambient_density * f32::powf(velocity.length(), 2.0) * drag_area
}

pub struct VolumetricBodyPlugin;

impl Plugin for VolumetricBodyPlugin {
    fn build(&self, _app: &mut App) {
        // TODO: Add systems to update volume from meshes
    }
}

/// The volume of a body.
#[derive(
    Reflect,
    Clone,
    Copy,
    Component,
    Debug,
    Default,
    Deref,
    DerefMut,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[reflect(Serialize, Deserialize)]
pub struct Volume(pub Scalar);

impl Volume {
    /// Zero volume.
    pub const ZERO: Self = Self(0.0);
}

impl From<&Mesh> for Volume {
    fn from(mesh: &Mesh) -> Self {
        compute_volume_from_mesh(mesh)
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

#[derive(Component, Debug)]
pub struct VolumetricBody {
    mesh: Mesh,
    pub mass: Mass,
    pub volume: Volume,
}

impl VolumetricBody {
    fn new(mesh: Mesh, mass: Mass) -> Self {
        let volume = Volume::from(&mesh);
        Self { mesh, mass, volume }
    }

    fn density(&self) -> Density {
        Density::new(self.mass, self.volume)
    }

    fn update_mesh(&mut self, mesh: Mesh) {
        self.mesh = mesh;
        self.volume = compute_volume_from_mesh(&self.mesh);
    }
}

fn compute_volume_from_mesh(mesh: &Mesh) -> Volume {
    // TODO: Implement
    Volume::ZERO
}
