use avian3d::prelude::*;
use bevy::prelude::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, simple_scene);
    }
}

/// set up a simple 3D scene
fn simple_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // light
    commands.spawn((
        Name::new("Light"),
        PointLightBundle {
            point_light: PointLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        },
    ));
    // camera
    commands.spawn((
        Name::new("Camera"),
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
    // ground
    let ground_size = Vec3::new(8.0, 0.1, 8.0);
    commands.spawn((
        Name::new("Ground"),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(ground_size.x, ground_size.y, ground_size.z)),
            material: materials.add(Color::srgb(0.75, 0.75, 0.75)),
            transform: Transform::from_translation(Vec3::new(0.0, -0.05, 0.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(ground_size.x / 2.0, ground_size.y / 2.0, ground_size.z / 2.0),
    ));
}