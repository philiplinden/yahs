//! Properties, attributes and functions related to the balloon.

use avian3d::{math::PI, prelude::*};
use bevy::prelude::*;

use crate::{
    debug,
    forces::{BuoyancyForce, DragForce, Forces, WeightForce},
    gas::IdealGas,
    geometry::{shell_volume, sphere_radius_from_volume},
    units::{VolumeUnit, MassUnit},
};


pub(crate) fn plugin(app: &mut App) {
    app.register_type::<Balloon>();
    app.register_type::<Skin>();
}

/// The balloon is a surface that contains an [`IdealGas`]. [`Balloon`]
/// is a dynamic [`RigidBody`] with [`Forces`].
/// The [`Balloon`] can have an [`ArbitraryShape`] that can be updated based on the
/// pressure of the [`IdealGas`] it contains, like to account for stretching.
#[derive(Component, Debug, Reflect, Clone)]
#[require(RigidBody(|| RigidBody::Dynamic), Mass, Forces, WeightForce, DragForce, BuoyancyForce)]
pub struct Balloon {
    // The 3d shape of the balloon constructed from a [`PrimitiveShape`].
    // TODO: Accept other shapes that implement [`Measured3d`]
    pub mesh: Handle<Mesh>,
    pub skin: Skin,
    pub gas: IdealGas,
}

impl Default for Balloon {
    fn default() -> Self {
        Balloon {
            mesh: Handle::default(), // Use default instead of placeholder
            skin: Skin::default(),
            gas: IdealGas::default(),
        }
    }
}

/// The skin is the material that composes the outer surface of the balloon.
/// TODO: Implement multiple material types, such as latex, polyurethane, etc.
#[derive(Debug, Clone, Reflect)]
pub struct Skin {
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

impl Default for Skin {
    fn default() -> Self {
        Skin {
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

impl Skin {
    pub fn mass_for_surface_area(&self, surface_area: f32) -> MassUnit {
        MassUnit(surface_area * self.density * self.thickness)
    }
}
