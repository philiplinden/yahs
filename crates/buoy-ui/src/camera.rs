use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use big_space::{prelude::*, camera::CameraInput};
use buoy_core::prelude::GridPrecision;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        big_space::camera::CameraControllerPlugin::<GridPrecision>::default(),
    ));
    app.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
    });
    app.add_systems(PostStartup, setup_camera)
        .add_systems(PostUpdate, (cursor_grab_system, big_space::camera::default_camera_inputs));
}

fn setup_camera(
    mut commands: Commands,
    origin: Query<Entity, With<FloatingOrigin>>,
) {
    let origin = origin.single();
    commands.entity(origin).insert((
            Camera3d::default(),
            Transform::from_xyz(0.0, 4.0, 22.0).looking_at(Vec3::ZERO, Vec3::Y),
            big_space::camera::CameraController::default() // Built-in camera controller
                .with_speed_bounds([0.1, 10e35])
                .with_smoothness(0.98, 0.98)
                .with_speed(1.0),
    ));
}

fn cursor_grab_system(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut cam: ResMut<CameraInput>,
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let Some(mut window) = windows.get_single_mut().ok() else {
        return;
    };

    if btn.just_pressed(MouseButton::Left) {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
        // window.mode = WindowMode::BorderlessFullscreen;
        cam.defaults_disabled = false;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor_options.grab_mode = CursorGrabMode::None;
        window.cursor_options.visible = true;
        // window.mode = WindowMode::Windowed;
        cam.defaults_disabled = true;
    }
}
