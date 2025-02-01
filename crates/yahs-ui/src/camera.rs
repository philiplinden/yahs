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
        app.add_plugins((CameraControllerPlugin, CameraTargetingPlugin));
    }
}

/// The main camera component.
#[derive(Component, Default, Reflect)]
#[require(Camera3d, PerspectiveProjection, CameraController, Transform)]
pub struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("Main Camera"),
        MainCamera,
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));
}

struct CameraTargetingPlugin;

impl Plugin for CameraTargetingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraTarget>();
        app.register_type::<Targetable>();
        app.add_systems(Update, target_the_first_thing_you_find);
    }
}

#[derive(Component, Debug, Reflect)]
pub struct Targetable;

#[derive(Resource, Debug)]
pub struct CameraTarget(Option<Vec3>);

impl Default for CameraTarget {
    fn default() -> Self {
        Self(None)
    }
}

impl CameraTarget {
    pub fn set(&mut self, target: Vec3) {
        self.0 = Some(target);
    }

    pub fn get(&self) -> Option<Vec3> {
        self.0
    }
}

fn target_the_first_thing_you_find(
    mut camera_target: ResMut<CameraTarget>,
    targets: Query<&Transform, Added<Targetable>>,
) {
    if let Ok(target) = targets.get_single() {
        camera_target.set(target.translation);
    }
}

struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, run_camera_controller);
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

#[allow(clippy::too_many_arguments)]
fn run_camera_controller(
    mut windows: Query<&mut Window>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    query: Single<(&mut Transform, &CameraController), With<MainCamera>>,
    targeted: Res<CameraTarget>,
) {
    let (mut camera_transform, camera_controller) = query.into_inner();

    // Handle key input
    let mut axis_input = Vec3::ZERO;
    if key_input.pressed(camera_controller.controls.tap_forward) {
        axis_input.z += 1.0;
    }
    if key_input.pressed(camera_controller.controls.tap_back) {
        axis_input.z -= 1.0;
    }

    if key_input.pressed(camera_controller.controls.tap_right) {
        axis_input.x += 1.0;
    }
    if key_input.pressed(camera_controller.controls.tap_left) {
        axis_input.x -= 1.0;
    }
    if key_input.pressed(camera_controller.controls.tap_up) {
        axis_input.y += 1.0;
    }

    if key_input.pressed(camera_controller.controls.tap_down) {
        axis_input.y -= 1.0;
    }

    let looking = mouse_button_input.pressed(camera_controller.controls.hold_look);

    // Handle cursor grab
    if looking {
        for mut window in &mut windows {
            if !window.focused {
                continue;
            }

            window.cursor_options.grab_mode = CursorGrabMode::Locked;
            window.cursor_options.visible = false;
        }
    } else {
        for mut window in &mut windows {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
        }
    }

    let look_target = match targeted.get() {
        Some(target) => target,
        None => camera_transform.translation,
    };

    // Handle mouse input
    if accumulated_mouse_motion.delta != Vec2::ZERO && looking {
        let delta_pitch =
            (accumulated_mouse_motion.delta.y * RADIANS_PER_DOT * camera_controller.sensitivity)
                .clamp(-PI / 2., PI / 2.);
        let delta_yaw =
            accumulated_mouse_motion.delta.x * RADIANS_PER_DOT * camera_controller.sensitivity;

        // Calculate the current distance from target
        let current_distance = (camera_transform.translation - look_target).length();
        
        // Rotate around target
        camera_transform.rotate_around(
            look_target,
            Quat::from_euler(EulerRot::YXZ, -delta_yaw, -delta_pitch, 0.0),
        );

        // Maintain distance from target
        let direction = (camera_transform.translation - look_target).normalize();
        camera_transform.translation = look_target + direction * current_distance;
    }

    // Handle scroll zoom
    let scroll_amount = match accumulated_mouse_scroll.unit {
        MouseScrollUnit::Line => accumulated_mouse_scroll.delta.y,
        MouseScrollUnit::Pixel => accumulated_mouse_scroll.delta.y / 16.0,
    };

    if scroll_amount != 0.0 {
        let direction = (camera_transform.translation - look_target).normalize();
        let new_distance = (camera_transform.translation - look_target).length() - 
            scroll_amount * camera_controller.scroll_factor;
        camera_transform.translation = look_target + direction * new_distance;
    }
}
