use avian3d::prelude::*;
use bevy::prelude::*;
use std::f32::consts::PI;

use yahs::prelude::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_lighting, spawn_balloon));
        app.add_systems(Update, animate_light_direction);
        // app.add_systems(PostStartup, |mut commands: Commands| {
        //     commands.set_state(SimState::Running);
        // });
    }
}

fn spawn_balloon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.0, 0.0),
        ..default()
    });
    let sphere = Sphere::default();
    let shape = meshes.add(sphere.mesh().ico(5).unwrap());
    let species = GasSpecies::helium();
    commands.spawn((
        Name::new("Balloon"),
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
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
        MeshMaterial3d(debug_material),
        Mesh3d(shape),
    ));
}

fn setup_lighting(mut commands: Commands) {
    // directional 'sun' light
    commands.spawn((
        DirectionalLight {
            illuminance: 32000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 2.0, 0.0).with_rotation(Quat::from_rotation_x(-PI / 4.)),
    ));
    // ambient light
    // NOTE: The ambient light is used to scale how bright the environment map is so with a bright
    // environment map, use an appropriate color and brightness to match
    commands.insert_resource(AmbientLight {
        color: Color::srgb_u8(210, 220, 240),
        brightness: 1.0,
    });
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() * 0.5);
    }
}
