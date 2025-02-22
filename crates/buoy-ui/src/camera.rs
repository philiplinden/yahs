use bevy::{
    math::DVec3,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use big_space::{camera::CameraInput, prelude::*};
use buoy_core::prelude::{Precision, RootGrid};

pub fn plugin(app: &mut App) {
    app.add_plugins((big_space::camera::CameraControllerPlugin::<Precision>::default(),));
    app.add_systems(PostStartup, setup_camera).add_systems(
        PostUpdate,
        (cursor_grab_system, big_space::camera::default_camera_inputs),
    );
}

/// Spawns the camera in the root grid and attaches the FloatingOrigin to it.
///
/// This system removes the previous FloatingOrigin and spawns a new one with
/// the camera. There should only be one FloatingOrigin. Any spatial entity can
/// be the floating origin. Attaching it to the camera ensures the camera will
/// never see floating point precision rendering artifacts.
fn setup_camera(
    mut commands: Commands,
    mut previous_origin: Query<Entity, With<FloatingOrigin>>,
    // HACK: This is a hack to access the root grid and add to it. It is not
    // ideal nor recommended. https://github.com/aevyrie/big_space/issues/36
    root_grid: Query<(Entity, &Grid<Precision>), With<RootGrid>>,
) {
    // Remove the FloatingOrigin component from the previous origin.
    let origin = previous_origin.single_mut();
    commands.entity(origin).remove::<FloatingOrigin>();

    let (root_grid_id, root_grid) = root_grid.single();

    // Spawn the camera and attach the FloatingOrigin to it.
    let object_pos = DVec3::new(0.0, 10.0, 20.0);
    let (object_cell, object_pos) = root_grid.translation_to_grid(object_pos);
    commands
        .spawn((
            Camera3d::default(),
            FloatingOrigin,
            object_cell,
            Transform::from_translation(object_pos).looking_at(Vec3::ZERO, Vec3::Y),
            big_space::camera::CameraController::default() // Built-in camera controller
                .with_speed_bounds([0.1, 10e35])
                .with_smoothness(0.98, 0.98)
                .with_speed(1.0),
        ))
        .set_parent(root_grid_id);
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
