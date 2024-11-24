mod camera;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::simulator::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(camera::CameraPlugin);
        app.add_systems(Startup, (simple_scene, spawn_balloon));
    }
}

/// set up a simple 3D scene
fn simple_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_size = Vec3::new(50.0, 0.1, 50.0);
    let plane = meshes.add(Plane3d::default().mesh().size(ground_size.x,  ground_size.z).subdivisions(10));
    let plane_material = materials.add(StandardMaterial::default());

    // light
    commands.spawn((
        Name::new("Light"),
        PointLight {
            intensity: 1500.0,
            ..default()
        },
    ));

    // ground
    commands.spawn((
        Name::new("Ground"),
        RigidBody::Static,
        Collider::cuboid(ground_size.x, ground_size.y, ground_size.z),
        Mesh3d(plane.clone()),
        MeshMaterial3d(plane_material.clone()),
    ));

    // ceiling
    commands.spawn((
        Name::new("Ceiling"),
        RigidBody::Static,
        Collider::cuboid(ground_size.x, ground_size.y, ground_size.z),
        Mesh3d(plane.clone()),
        MeshMaterial3d(plane_material.clone()),
    ));
}

pub fn spawn_balloon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.0, 0.0),
        ..default()
    });
    let shape = meshes.add(Sphere::default().mesh().ico(5).unwrap());
    commands.spawn((
        Name::new("Balloon"),
        SimulatedBody,
        BalloonBundle {
            balloon: Balloon::default(),
            gas: IdealGasBundle {
                species: GasSpecies::helium(),
                gas: IdealGas {
                    temperature: Temperature::STANDARD,
                    pressure: Pressure::STANDARD,
                    mass: Mass::new(1.0),
                },
            },
        },
        RigidBody::Dynamic,
        Mesh3d(shape),
        MeshMaterial3d(debug_material),
    ));

}
