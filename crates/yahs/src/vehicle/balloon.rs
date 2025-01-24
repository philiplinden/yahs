//! Properties, attributes and functions related to the balloon.

use avian3d::{math::PI, prelude::*};
use bevy::prelude::*;

use crate::{
    gas::IdealGas,
    forces::{Weight, Drag, Buoyancy},
    geometry::{Volume, sphere_radius_from_volume}
};

pub struct BalloonPlugin;

impl Plugin for BalloonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_balloon_from_gas,
        );
    }
}

/// The balloon is a surface that contains an [`IdealGas`]. [`Balloon`]
/// is a dynamic [`RigidBody`] with [`Weight`], [`Drag`], and [`Buoyancy`] forces.
/// The [`Balloon`] can have an [`ArbitraryShape`] that can be updated based on the
/// pressure of the [`IdealGas`] it contains, like to account for stretching.
#[derive(Component, Debug, Clone, PartialEq)]
#[require(IdealGas, RigidBody(|| RigidBody::Dynamic), Weight, Drag, Buoyancy)]
pub struct Balloon {
    // The 3d shape of the balloon constructed from a [`PrimitiveShape`].
    // TODO: Accept other shapes that implement [`Measured3d`]
    pub shape: Sphere,
    pub envelope: Envelope,
}

impl Default for Balloon {
    fn default() -> Self {
        Balloon {
            shape: Sphere::new(1.0),
            envelope: Envelope::default(),
        }
    }
}

impl Balloon {
    fn set_volume(&mut self, volume: &Volume) {
        self.shape.radius = sphere_radius_from_volume(volume.m3());
    }
}

/// The envelope is the material that composes the outer surface of the balloon.
/// TODO: Implement multiple material types, such as latex, polyurethane, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct Envelope {
    // temperature (K) where the given material fails
    pub max_temperature: f32,
    // density (kg/m³) of the envelope material
    pub density: f32,
    // how much thermal radiation is emitted
    pub emissivity: f32,
    // how much thermal radiation is absorbed
    pub absorptivity: f32,
    // thermal conductivity (W/mK) of the material at room temperature
    pub thermal_conductivity: f32,
    // J/kgK
    pub specific_heat: f32,
    // ratio of change in width for a given change in length
    pub poissons_ratio: f32,
    // Youngs Modulus aka Modulus of Elasticity (Pa)
    pub elasticity: f32,
    // elongation at failure (decimal, unitless) 1 = original size
    pub max_strain: f32,
    // tangential stress at failure (Pa)
    pub max_stress: f32,
    // thickness of the envelope material (m)
    pub thickness: f32,
}

impl Default for Envelope {
    fn default() -> Self {
        Envelope {
            max_temperature: 373.0,
            density: 920.0,
            emissivity: 0.9,
            absorptivity: 0.9,
            thermal_conductivity: 0.13,
            specific_heat: 2000.0,
            poissons_ratio: 0.5,
            elasticity: 0.01e9,
            max_strain: 0.8,
            max_stress: 0.5e6,
            thickness: 0.0001,
        }
    }
}

fn update_balloon_from_gas(mut query: Query<(&mut Balloon, &Volume), With<IdealGas>>) {
    for (mut balloon, volume) in query.iter_mut() {
        balloon.set_volume(volume);
    }
}
