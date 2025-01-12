use bevy::prelude::*;
use avian3d::prelude::*;
use crate::{
    balloon::{BalloonBundle, BalloonMaterial, Balloon},
    ideal_gas::{GasSpecies, IdealGas},
};

pub fn spawn_balloon(mut commands: Commands) {
    let species = GasSpecies::helium();
    commands.spawn((
        Name::new("Balloon"),
        BalloonBundle {
            balloon: Balloon {
                material_properties: BalloonMaterial::default(),
                shape: Sphere::default(),
            },
            gas: IdealGas::new(species).with_mass(Mass(0.01)),
        },
    ));
}
