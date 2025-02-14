use avian3d::prelude::*;
use bevy::prelude::*;
use std::f32::consts::PI;

use super::camera::CameraAttachment;
use yahs::prelude::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                setup_lighting,
                spawn_balloon,
                // (spawn_balloon, spawn_payload, spawn_tether).chain(),
            ),
        );
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
    commands
        .spawn((
            Name::new("Balloon"),
            Balloon {
                gas: IdealGas::default().with_mass(1.0),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 3.0, 0.0)),
            MeshMaterial3d(debug_material.clone()),
            Mesh3d(shape),
            CameraAttachment::default(),

        ));
}

#[allow(dead_code)]
fn spawn_payload(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let payload_shape = Cuboid::new(1.0, 1.0, 1.0);
    let debug_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.0, 1.0),
        ..default()
    });
    commands.spawn((
        PayloadBundle::new(
            payload_shape,
            Mass(0.01),
            Transform::from_translation(Vec3::new(0.0, 2.0, 0.0)),
        ),
        MeshMaterial3d(debug_material.clone()),
        Mesh3d(meshes.add(payload_shape.mesh())),
    ));
}

#[allow(dead_code)]
fn spawn_tether(
    mut commands: Commands,
    balloon_entity: Query<Entity, With<Balloon>>,
    payload_entity: Query<Entity, With<Payload>>,
) {
    Tether::link_entities(
        &mut commands,
        10.0,
        balloon_entity.get_single().unwrap(),
        payload_entity.get_single().unwrap(),
    );
}

#[allow(dead_code)]
fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_shape = meshes.add(Cuboid::new(100.0, 0.1, 100.0).mesh());
    let ground_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.1, 0.),
        ..default()
    });
    commands.spawn((
        Transform::from_xyz(0.0, -1.0, 0.0),
        Collider::cuboid(100.0, 0.1, 100.0),
        RigidBody::Static,
        Mesh3d(ground_shape),
        MeshMaterial3d(ground_material),
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
