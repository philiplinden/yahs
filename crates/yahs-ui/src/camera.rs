use avian3d::prelude::*;
use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit},
    prelude::*,
    window::CursorGrabMode,
};
use std::f32::consts::PI;

use super::controls::CameraControls;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_plugins(CameraControllerPlugin);
    }
}

/// The main camera component.
#[derive(Component, Default, Reflect)]
#[require(
    Camera3d,
    PerspectiveProjection,
    CameraController,
    Transform,
    TransformInterpolation,
    RotationInterpolation,
)]
pub struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("Main Camera"),
        MainCamera,
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));
}

struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                handle_cursor_grab,
                handle_camera_rotation,
                handle_camera_zoom,
                update_camera_position,
            )
                .chain(),
        );
    }
}

#[derive(Component)]
struct CameraController {
    pub controls: CameraControls,
    pub sensitivity: f32,
    pub scroll_factor: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            controls: CameraControls::default(),
            sensitivity: 1.0,
            scroll_factor: 0.1,
        }
    }
}

/// Based on Valorant's default sensitivity, not entirely sure why it is exactly
/// 1.0 / 180.0, but I'm guessing it is a misunderstanding between
/// degrees/radians and then sticking with it because it felt nice.
pub const RADIANS_PER_DOT: f32 = 1.0 / 180.0;

#[derive(Component)]
pub struct CameraAttachment {
    pub relative_pos: Vec3,
}

impl Default for CameraAttachment {
    fn default() -> Self {
        Self {
            relative_pos: Vec3::new(0.0, 0.0, 10.0),
        }
    }
}

fn update_camera_position(
    mut camera: Query<&mut Transform, With<MainCamera>>,
    attachments: Query<(&CameraAttachment, &Transform), Without<MainCamera>>,
) {
    let mut camera_transform = camera.single_mut();

    if let Ok((attachment, attached_transform)) = attachments.get_single() {
        camera_transform.translation = attached_transform.translation + attachment.relative_pos;
    }
}

fn handle_cursor_grab(
    mut windows: Query<&mut Window>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera: Query<&CameraController, With<MainCamera>>,
) {
    let camera_controller = camera.single();
    let looking = mouse_button_input.pressed(camera_controller.controls.hold_look);

    for mut window in &mut windows {
        if looking && window.focused {
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
            window.cursor_options.visible = false;
        } else {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
        }
    }
}

fn handle_camera_rotation(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut camera: Query<(&CameraController, &mut Transform), With<MainCamera>>,
    mut attachments: Query<(&mut CameraAttachment, &Transform), Without<MainCamera>>,
) {
    let (camera_controller, mut camera_transform) = camera.single_mut();
    let looking = mouse_button_input.pressed(camera_controller.controls.hold_look);
    if !looking || accumulated_mouse_motion.delta == Vec2::ZERO {
        return;
    }

    // Try to get a single camera attachment. This will:
    // - Return Ok(attachment) if exactly one attachment exists
    // - Return Err if zero or multiple attachments exist
    let (mut attachment, host_transform) = if let Ok(a) = attachments.get_single_mut() {
        a
    } else {
        // Exit the function if we don't have exactly one attachment
        return;
    };

    let delta_pitch =
        (accumulated_mouse_motion.delta.y * RADIANS_PER_DOT * camera_controller.sensitivity)
            .clamp(-PI / 2., PI / 2.);
    let delta_yaw =
        accumulated_mouse_motion.delta.x * RADIANS_PER_DOT * camera_controller.sensitivity;

    // Rotate the relative position vector
    attachment.relative_pos = Quat::from_euler(EulerRot::YXZ, -delta_yaw, -delta_pitch, 0.0)
        .mul_vec3(attachment.relative_pos);

    // Look at the attached object's position
    camera_transform.look_at(host_transform.translation, Vec3::Y);
}

fn handle_camera_zoom(
    accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
    camera: Query<&CameraController, With<MainCamera>>,
    mut attachments: Query<&mut CameraAttachment, Without<MainCamera>>,
) {
    let camera_controller = camera.single();

    let scroll_amount = match accumulated_mouse_scroll.unit {
        MouseScrollUnit::Line => accumulated_mouse_scroll.delta.y,
        MouseScrollUnit::Pixel => accumulated_mouse_scroll.delta.y / 16.0,
    };

    if scroll_amount == 0.0 {
        return;
    }

    // Try to get a single camera attachment. This will:
    // - Return Ok(attachment) if exactly one attachment exists
    // - Return Err if zero or multiple attachments exist
    let mut attachment = if let Ok(a) = attachments.get_single_mut() {
        a
    } else {
        // Exit the function if we don't have exactly one attachment
        return;
    };

    // Scale the relative position vector
    let new_length =
        attachment.relative_pos.length() - scroll_amount * camera_controller.scroll_factor;
    attachment.relative_pos = attachment.relative_pos.normalize() * new_length;
}
