use avian3d::prelude::*;
use bevy::prelude::*;

use crate::simulator::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_balloon, simple_scene));
        // app.add_systems(PostStartup, |mut commands: Commands| {
        //     commands.set_state(SimState::Running);
        // });
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

fn spawn_balloon(
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
    let sphere = Sphere::default();
    let shape = meshes.add(sphere.mesh().ico(5).unwrap());
    let species = GasSpecies::helium();
    commands.spawn((
        Name::new("Balloon"),
        SimulatedBody,
        BalloonBundle {
            balloon: Balloon {
                material_properties: BalloonMaterial::default(),
                shape: sphere,
            },
            gas: IdealGas::new(species).with_mass(Mass::new(0.01)),
        },
        RigidBody::Dynamic,
        Collider::sphere(sphere.radius),
        Transform {
            translation: Vec3::new(0.0, 10.0, 0.0),
            ..default()
        },
        MeshMaterial3d(debug_material),
        Mesh3d(shape),
    ));
}
