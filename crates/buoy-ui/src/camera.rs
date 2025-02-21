use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use big_space::camera::CameraInput;
use buoy_core::prelude::GridPrecision;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        big_space::camera::CameraControllerPlugin::<GridPrecision>::default(),
    ));
    app.add_systems(PreUpdate, cursor_grab_system);
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
