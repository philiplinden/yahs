use bevy::prelude::*;

// use crate::controls::CameraControls;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        // Note we're setting the initial position below with yaw, pitch, and radius, hence
        // we don't set transform on the camera.
        Camera3d::default(),
        Transform::from_xyz(0.0, 20., 50.0).looking_at(Vec3::new(0., 20., 0.), Vec3::Y),
    ));
}
