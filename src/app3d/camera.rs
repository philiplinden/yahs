use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit},
    prelude::*,
    window::CursorGrabMode,
};
use std::f32::consts::PI;

use super::controls::{CameraControls, KeyBindingsConfig};
use crate::simulator::Balloon;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_plugins(CameraTargetPlugin);
        app.add_plugins(CameraControllerPlugin);
    }
}

/// The main camera component.
#[derive(Component, Default, Reflect)]
#[require(Camera3d, PerspectiveProjection, CameraController)]
pub struct MainCamera;

fn setup(mut commands: Commands, keybinds: Res<KeyBindingsConfig>) {
    commands.spawn((
        Name::new("Main Camera"),
        MainCamera,
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));
    let controls = &keybinds.into_inner().camera_controls;
    commands.spawn((
        Text::new(format!(
            "Freecam Controls:
    {:?} - Focus on target
    {:?} - Cycle to next target & follow it
    {:?} - Clear target
    {:?} Mouse - Click & drag to rotate camera
    {:?} / {:?} / {:?} / {:?} - Fly forward, backward, left, right
    {:?} / {:?} - Fly up / down
    {:?} - Hold to fly faster",
            controls.tap_focus_target,
            controls.tap_cycle_target,
            controls.tap_clear_target,
            controls.hold_look,
            controls.tap_forward,
            controls.tap_back,
            controls.tap_left,
            controls.tap_right,
            controls.tap_up,
            controls.tap_down,
            controls.tap_run,
        )),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.),
            left: Val::Px(12.),
            ..default()
        },
    ));
}

struct CameraTargetPlugin;

impl Plugin for CameraTargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CycleToNextTarget>();
        app.add_observer(cycle_to_next_target_system);

        app.add_event::<FocusOnTarget>();
        app.add_observer(focus_on_target);

        app.add_event::<ClearTarget>();
        app.add_observer(clear_target);

        app.add_systems(
            Update,
            (
                mark_new_targetables,
                follow_selected_target,
                handle_camera_targeting_inputs,
            ),
        );
    }
}

const DEFAULT_TARGET_OFFSET: Vec3 = Vec3::new(0.0, 0.0, 10.0);

#[derive(Event)]
struct FocusOnTarget;

/// A marker component for entities that can be selected as a camera target.
#[derive(Component, Default, Reflect)]
pub struct Targetable;

/// A marker component for entities that can be selected as a camera target.
#[derive(Component, Default, Reflect)]
pub struct Targeted;

/// An event that is emitted when the camera should cycle to the next target.
#[derive(Event)]
struct CycleToNextTarget;

/// An event that is emitted when the camera should clear its target.
#[derive(Event)]
struct ClearTarget;

fn mark_new_targetables(mut commands: Commands, balloons: Query<Entity, Added<Balloon>>) {
    for entity in &balloons {
        commands.entity(entity).insert(Targetable);
    }
}

fn handle_camera_targeting_inputs(
    mut commands: Commands,
    key_input: Res<ButtonInput<KeyCode>>,
    keybinds: Res<KeyBindingsConfig>,
) {
    let controls = &keybinds.into_inner().camera_controls;
    if key_input.just_pressed(controls.tap_focus_target) {
        commands.trigger(FocusOnTarget);
    }
    if key_input.just_pressed(controls.tap_cycle_target) {
        commands.trigger(CycleToNextTarget);
    }
    if key_input.just_pressed(controls.tap_clear_target) {
        commands.trigger(ClearTarget);
    }
}

fn clear_target(
    _trigger: Trigger<ClearTarget>,
    mut commands: Commands,
    target: Option<Single<Entity, With<Targeted>>>,
) {
    if let Some(target) = target {
        commands.entity(target.into_inner()).remove::<Targeted>();
    }
}

fn focus_on_target(
    _trigger: Trigger<FocusOnTarget>,
    target: Option<Single<&Transform, (With<Targeted>, Without<MainCamera>)>>,
    camera: Single<&mut Transform, With<MainCamera>>,
) {
    if let Some(target) = target {
        let target = target.into_inner();
        let mut cam = camera.into_inner();
        cam.look_at(target.translation, Vec3::Y);
    }
}

fn follow_selected_target(
    target: Option<Single<&Transform, (With<Targeted>, Without<MainCamera>)>>,
    camera: Single<&mut Transform, (With<MainCamera>, Without<Targeted>)>,
) {
    let mut cam = camera.into_inner();
    if let Some(target) = target {
        let target = target.into_inner();
        cam.translation = cam.translation + target.translation;
    }
}

fn cycle_to_next_target_system(
    _trigger: Trigger<CycleToNextTarget>,
    mut commands: Commands,
    old_target: Option<Single<Entity, With<Targeted>>>,
    targets: Query<
        (Entity, &Transform),
        (With<Targetable>, Without<Targeted>, Without<MainCamera>),
    >,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    let mut camera = camera_query.single_mut();

    let target_entities: Vec<(Entity, &Transform)> = targets.iter().collect();
    if target_entities.is_empty() {
        return;
    }

    let old_target = match old_target {
        Some(target) => Some(target.into_inner()),
        None => None,
    };

    // Determine the next target index
    let next_target = if let Some(current_entity) = old_target {
        let current_index = target_entities
            .iter()
            .position(|&(e, _)| e == current_entity)
            .unwrap_or(0);
        target_entities
            .iter()
            .cycle()
            .nth(current_index + 1)
            .map(|&(e, _)| e)
            .unwrap_or(target_entities[0].0)
    } else {
        target_entities[0].0
    };

    // Remove Targeted from the current entity, if any
    if let Some(current_entity) = old_target {
        commands.entity(current_entity).remove::<Targeted>();
    }

    // Add Targeted to the next entity
    commands.entity(next_target).insert(Targeted);

    // Retrieve the Transform of the new target and update the camera position
    if let Ok((_, target_transform)) = targets.get(next_target) {
        camera.translation = target_transform.translation + DEFAULT_TARGET_OFFSET;
        camera.look_at(target_transform.translation, Vec3::Y);
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
    pub enabled: bool,
    pub initialized: bool,
    pub sensitivity: f32,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub scroll_factor: f32,
    pub friction: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
    pub controls: CameraControls,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            initialized: false,
            sensitivity: 1.0,
            walk_speed: 10.0,
            run_speed: 30.0,
            scroll_factor: 0.1,
            friction: 0.1,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
            controls: CameraControls::default(),
        }
    }
}

/// Based on Valorant's default sensitivity, not entirely sure why it is exactly
/// 1.0 / 180.0, but I'm guessing it is a misunderstanding between
/// degrees/radians and then sticking with it because it felt nice.
pub const RADIANS_PER_DOT: f32 = 1.0 / 180.0;

#[allow(clippy::too_many_arguments)]
fn run_camera_controller(
    time: Res<Time>,
    mut windows: Query<&mut Window>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    query: Single<(&mut Transform, &mut CameraController), With<MainCamera>>,
) {
    let dt = time.delta_secs();

    let (mut transform, mut controller) = query.into_inner();

    if !controller.initialized {
        let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
        controller.yaw = yaw;
        controller.pitch = pitch;
        controller.initialized = true;
    }
    if !controller.enabled {
        return;
    }

    let mut scroll = 0.0;

    let amount = match accumulated_mouse_scroll.unit {
        MouseScrollUnit::Line => accumulated_mouse_scroll.delta.y,
        MouseScrollUnit::Pixel => accumulated_mouse_scroll.delta.y / 16.0,
    };
    scroll += amount;
    controller.walk_speed += scroll * controller.scroll_factor * controller.walk_speed;
    controller.run_speed = controller.walk_speed * 3.0;

    // Handle key input
    let mut axis_input = Vec3::ZERO;
    if key_input.pressed(controller.controls.tap_forward) {
        axis_input.z += 1.0;
    }
    if key_input.pressed(controller.controls.tap_back) {
        axis_input.z -= 1.0;
    }
    if key_input.pressed(controller.controls.tap_right) {
        axis_input.x += 1.0;
    }
    if key_input.pressed(controller.controls.tap_left) {
        axis_input.x -= 1.0;
    }
    if key_input.pressed(controller.controls.tap_up) {
        axis_input.y += 1.0;
    }
    if key_input.pressed(controller.controls.tap_down) {
        axis_input.y -= 1.0;
    }

    let looking = mouse_button_input.pressed(controller.controls.hold_look);

    // Apply movement update
    if axis_input != Vec3::ZERO {
        let max_speed = if key_input.pressed(controller.controls.tap_run) {
            controller.run_speed
        } else {
            controller.walk_speed
        };
        controller.velocity = axis_input.normalize() * max_speed;
    } else {
        let friction = controller.friction.clamp(0.0, 1.0);
        controller.velocity *= 1.0 - friction;
        if controller.velocity.length_squared() < 1e-6 {
            controller.velocity = Vec3::ZERO;
        }
    }
    let forward = *transform.forward();
    let right = *transform.right();
    transform.translation += controller.velocity.x * dt * right
        + controller.velocity.y * dt * Vec3::Y
        + controller.velocity.z * dt * forward;

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

    // Handle mouse input
    if accumulated_mouse_motion.delta != Vec2::ZERO && looking {
        // Apply look update
        controller.pitch = (controller.pitch
            - accumulated_mouse_motion.delta.y * RADIANS_PER_DOT * controller.sensitivity)
            .clamp(-PI / 2., PI / 2.);
        controller.yaw -=
            accumulated_mouse_motion.delta.x * RADIANS_PER_DOT * controller.sensitivity;
        transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, controller.yaw, controller.pitch);
    }
}
