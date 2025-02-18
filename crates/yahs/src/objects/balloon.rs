//! Properties, attributes and functions related to the balloon.

use avian3d::{math::PI, prelude::*};
use bevy::prelude::*;

use crate::{
    atmosphere::Atmosphere,
    ideal_gas::IdealGas,
    geometry::{shell_volume, sphere_radius_from_volume, sphere_surface_area, DragCoefficient},
    units::{AreaUnit, DensityUnit, MassUnit, VolumeUnit},
};

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<Balloon>();
    app.register_type::<Skin>();
    app.add_systems(PreUpdate, update_balloon_from_gas);
    app.add_systems(FixedUpdate, update_gas_from_atmosphere);
}


/// The balloon is a surface that contains an [`IdealGas`]. [`Balloon`]
/// is a dynamic [`RigidBody`] with [`Forces`].
/// The total mass of the balloon is the sum of the mass of the skin and the
/// mass of the gas.
#[derive(Component, Debug, Reflect, Clone)]
#[require(RigidBody(|| RigidBody::Dynamic), DragCoefficient)]
pub struct Balloon {
    // The 3d shape of the balloon constructed from a [`PrimitiveShape`].
    // TODO: Accept other shapes that implement [`Measured3d`]
    pub mesh: Handle<Mesh>,
    pub hack_volume: VolumeUnit,
    pub skin: Skin,
    pub gas: IdealGas,
}

impl Default for Balloon {
    fn default() -> Self {
        Balloon {
            mesh: Handle::default(), // Use default instead of placeholder
            hack_volume: VolumeUnit::ZERO,
            skin: Skin::default(),
            gas: IdealGas::default(),
        }
    }
}

impl Balloon {
    pub fn mass(&self) -> MassUnit {
        self.gas.mass + self.skin_mass()
    }

    pub fn volume(&self) -> VolumeUnit {
        self.hack_volume
    }

    pub fn update_volume(&mut self, volume: VolumeUnit) {
        // self.mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, volume.m3());
        self.hack_volume = volume;
    }

    pub fn density(&self) -> DensityUnit {
        self.mass() / self.volume()
    }

    pub fn skin_volume(&self) -> VolumeUnit {
        VolumeUnit::from_cubic_meters(shell_volume(
            sphere_radius_from_volume(self.volume().m3()),
            self.skin.thickness,
        ))
    }

    pub fn skin_mass(&self) -> MassUnit {
        MassUnit::from_kilograms(self.skin_volume().m3() * self.skin.density)
    }

    pub fn surface_area(&self) -> AreaUnit {
        AreaUnit::from_square_meters(sphere_surface_area(
            sphere_radius_from_volume(self.volume().m3()),
        ))
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

fn update_balloon_from_gas(mut balloon: Query<(&mut Balloon, &mut Mass)>) {
    for (mut balloon, mut mass) in balloon.iter_mut() {
        let volume = balloon.gas.volume();
        balloon.update_volume(volume);
        mass.0 = balloon.mass().kg();
    }
}

fn update_gas_from_atmosphere(mut query: Query<(&mut Balloon, &Position)>, atmosphere: Res<Atmosphere>) {
    for (mut balloon, position) in query.iter_mut() {
        balloon.gas.pressure = atmosphere.pressure(position.0);
        balloon.gas.temperature = atmosphere.temperature(position.0);
    }
}
