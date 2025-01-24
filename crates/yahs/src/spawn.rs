use bevy::prelude::*;
use avian3d::prelude::*;
use crate::{
    vehicle::Balloon,
    gas::IdealGasBundle,
};

pub fn spawn_balloon(mut commands: Commands) {
    commands.spawn((
        Name::new("Balloon"),
        Balloon::default(),
        IdealGasBundle::default(),
    ));
}
