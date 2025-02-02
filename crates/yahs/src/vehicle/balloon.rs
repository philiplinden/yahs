//! Properties, attributes and functions related to the balloon.

use avian3d::{math::PI, prelude::*};
use bevy::prelude::*;

use crate::{
    debug,
    forces::{BuoyancyForce, DragForce, Forces, WeightForce},
    gas::IdealGas,
    geometry::{shell_volume, sphere_radius_from_volume, Volume},
};

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<Balloon>();
    app.register_type::<Envelope>();
    app.add_systems(
        PreUpdate,
        (
            update_balloon_volume_from_gas,
            update_balloon_shape_from_volume,
            balloon_mass_from_envelope,
        )
            .chain(),
    );
}

/// The balloon is a surface that contains an [`IdealGas`]. [`Balloon`]
/// is a dynamic [`RigidBody`] with [`Forces`].
/// The [`Balloon`] can have an [`ArbitraryShape`] that can be updated based on the
/// pressure of the [`IdealGas`] it contains, like to account for stretching.
#[derive(Component, Debug, Reflect, Clone)]
#[require(RigidBody(|| RigidBody::Dynamic), Mass, Volume, Forces, WeightForce, DragForce)]
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
        let old_volume = self.shape.volume();
        if old_volume != volume.m3() {
            info!("Volume changing from {} to {}", old_volume, volume.m3());
            self.shape.radius = sphere_radius_from_volume(volume.m3());
        }
    }
}

/// The envelope is the material that composes the outer surface of the balloon.
/// TODO: Implement multiple material types, such as latex, polyurethane, etc.
#[derive(Debug, Clone, Reflect)]
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

fn balloon_mass_from_envelope(mut balloons: Query<(&mut Mass, &Balloon), Added<Balloon>>) {
    for (mut mass, balloon) in balloons.iter_mut() {
        mass.0 = balloon.envelope.density
            * shell_volume(balloon.envelope.thickness, balloon.shape.radius);
    }
}

fn update_balloon_volume_from_gas(
    mut balloons: Query<(&mut Volume, &Children), (With<Balloon>, Without<IdealGas>)>,
    gases: Query<&Volume, With<IdealGas>>,
) {
    for (mut balloon_volume, children) in balloons.iter_mut() {
        // Get the first child with IdealGas component
        for &child in children {
            if let Ok(gas_volume) = gases.get(child) {
                *balloon_volume = *gas_volume;
                break;
            }

        }
    }
}

fn update_balloon_shape_from_volume(mut balloons: Query<(&mut Balloon, &Volume), Changed<Volume>>) {
    for (mut balloon, volume) in balloons.iter_mut() {
        balloon.set_volume(volume);
    }
}
