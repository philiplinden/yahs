use avian3d::prelude::*;
use bevy::prelude::*;
use std::f32::consts::PI;

use yahs::{prelude::*, vehicle::balloon::Envelope};
use super::camera::CameraAttachment;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_lighting, spawn_balloon));
    }
}

fn spawn_balloon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    debug!("spawning balloon");
    let debug_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.0, 0.0),
        ..default()
    });
    let radius = 1.0;
    let sphere = Sphere { radius };
    let shape = meshes.add(sphere.mesh().ico(5).unwrap());
    let species = GasSpecies::helium();
    commands.spawn((
        Name::new("Balloon"),
        Balloon { shape: sphere, envelope: Envelope::default() },
        IdealGasBundle {
            species,
            mass: Mass(1.0),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
        MeshMaterial3d(debug_material),
        Mesh3d(shape),
        CameraAttachment::default(),
    ));
}

fn setup_lighting(mut commands: Commands) {
    debug!("spawning sunlight");
    commands.spawn((
        DirectionalLight {
            illuminance: 32000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 100.0, 0.0).with_rotation(Quat::from_rotation_x(-PI / 4.)),
    ));
    // ambient light
    // NOTE: The ambient light is used to scale how bright the environment map is so with a bright
    // environment map, use an appropriate color and brightness to match
    commands.insert_resource(AmbientLight {
        color: Color::srgb_u8(210, 220, 240),
        brightness: 1.0,
    });
}
