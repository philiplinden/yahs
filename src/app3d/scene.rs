use avian3d::prelude::*;
use bevy::prelude::*;

use crate::simulator::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
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
    let plane_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.5, 0.5),
        perceptual_roughness: 0.5,
        ..default()
    });

    // ground
    commands.spawn((
        Name::new("Ground"),
        RigidBody::Static,
        Mesh3d(plane.clone()),
        MeshMaterial3d(plane_material.clone()),
        ColliderConstructor::TrimeshFromMesh,
    ));
}

pub fn spawn_balloon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 0.0, 0.0, 0.5),
        perceptual_roughness: 0.0,
        metallic: 1.0,
        ..default()
    });
    let shape = meshes.add(Sphere::default().mesh().ico(5).unwrap());
    let species = GasSpecies::helium();
    commands.spawn((
        Name::new("Balloon"),
        SimulatedBody,
        Balloon,
        BalloonBundle {
            material_properties: BalloonMaterial::default(),
            mesh: Mesh3d(shape),
            gas: IdealGas::new(species),
        },
        RigidBody::Dynamic,
        ColliderConstructor::TrimeshFromMesh,
        Transform {
            translation: Vec3::new(0.0, 10.0, 0.0),
            ..default()
        },
        MeshMaterial3d(debug_material),
    ));
}
