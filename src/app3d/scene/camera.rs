use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use avian3d::math::TAU;

use crate::app3d::KeyBindingsConfig;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanOrbitCameraPlugin);
        app.add_systems(Startup, setup);
        app.add_systems(Update, toggle_camera_controls_system);
    }
}

fn setup(mut commands: Commands, key_bindings: Res<KeyBindingsConfig>) {
    commands.spawn((
        // Note we're setting the initial position below with yaw, pitch, and radius, hence
        // we don't set transform on the camera.
        Camera3dBundle::default(),
        PanOrbitCamera {
            // Set focal point (what the camera should look at)
            focus: Vec3::new(0.0, 1.0, 0.0),
            // Set the starting position, relative to focus (overrides camera's transform).
            yaw: Some(TAU / 8.0),
            pitch: Some(TAU / 8.0),
            radius: Some(5.0),
            // Set limits on rotation and zoom
            yaw_upper_limit: Some(TAU / 4.0),
            yaw_lower_limit: Some(-TAU / 4.0),
            pitch_upper_limit: Some(TAU / 3.0),
            pitch_lower_limit: Some(-TAU / 3.0),
            zoom_upper_limit: Some(100.0),
            zoom_lower_limit: 1.0,
            // Adjust sensitivity of controls
            orbit_sensitivity: 1.5,
            pan_sensitivity: 0.5,
            zoom_sensitivity: 0.5,
            // Allow the camera to go upside down
            allow_upside_down: true,
            // Change the controls (these match Blender)
            button_orbit: key_bindings.camera_controls.button_orbit,
            button_pan: key_bindings.camera_controls.button_pan,
            modifier_pan: key_bindings.camera_controls.modifier_pan,
            // Reverse the zoom direction
            reversed_zoom: false,
            ..default()
        },
    ));
}

// This is how you can change config at runtime.
// Press 'T' to toggle the camera zoom direction.
fn toggle_camera_controls_system(
    key_input: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindingsConfig>,
    mut pan_orbit_query: Query<&mut PanOrbitCamera>,
) {
    if key_input.just_pressed(key_bindings.camera_controls.toggle_zoom_direction) {
        for mut pan_orbit in pan_orbit_query.iter_mut() {
            pan_orbit.reversed_zoom = !pan_orbit.reversed_zoom;
        }
    }
}
