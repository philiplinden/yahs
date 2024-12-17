use bevy::prelude::*;

use yahs::prelude::{SimState, TimeScaleOptions};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KeyBindingsConfig>();
        app.add_plugins((PausePlayPlugin, ChangeTimeScalePlugin));
    }
}

#[allow(dead_code)]
#[derive(Resource, Default)]
pub struct KeyBindingsConfig {
    pub camera_controls: CameraControls,
    pub debug_controls: DebugControls,
    pub time_controls: TimeControls,
}

#[derive(Reflect)]
pub struct CameraControls {
    pub hold_look: MouseButton,
    pub tap_cycle_target: KeyCode,
    pub tap_clear_target: KeyCode,
    pub tap_focus_target: KeyCode,
    pub tap_forward: KeyCode,
    pub tap_back: KeyCode,
    pub tap_left: KeyCode,
    pub tap_right: KeyCode,
    pub tap_up: KeyCode,
    pub tap_down: KeyCode,
    pub tap_run: KeyCode,
}

#[derive(Reflect)]
pub struct DebugControls {
    pub toggle_1: KeyCode,
    pub toggle_2: KeyCode,
    pub toggle_3: KeyCode,
    pub toggle_4: KeyCode,
    pub toggle_5: KeyCode,
}

#[derive(Reflect)]
pub struct TimeControls {
    pub toggle_pause: KeyCode,
    pub faster: KeyCode,
    pub slower: KeyCode,
    pub reset_speed: KeyCode,
    pub toggle_real_time: KeyCode,
    pub scale_step: f32,
}

// ============================ DEFAULT KEYBINDINGS ============================

/// Defaults follow Blender conventions
impl Default for CameraControls {
    fn default() -> Self {
        Self {
            hold_look: MouseButton::Right,
            tap_cycle_target: KeyCode::Tab,
            tap_clear_target: KeyCode::KeyT,
            tap_focus_target: KeyCode::KeyF,
            tap_forward: KeyCode::KeyW,
            tap_back: KeyCode::KeyS,
            tap_left: KeyCode::KeyA,
            tap_right: KeyCode::KeyD,
            tap_up: KeyCode::KeyE,
            tap_down: KeyCode::KeyQ,
            tap_run: KeyCode::ShiftLeft,
        }
    }
}

impl Default for DebugControls {
    fn default() -> Self {
        Self {
            toggle_1: KeyCode::F1,
            toggle_2: KeyCode::F2,
            toggle_3: KeyCode::F3,
            toggle_4: KeyCode::F4,
            toggle_5: KeyCode::F5,
        }
    }
}

impl Default for TimeControls {
    fn default() -> Self {
        Self {
            toggle_pause: KeyCode::Space,
            faster: KeyCode::ArrowUp,
            slower: KeyCode::ArrowDown,
            reset_speed: KeyCode::Backspace,
            toggle_real_time: KeyCode::KeyR,
            scale_step: 0.1,
        }
    }
}

// ============================ CONTROL SYSTEMS ================================

struct PausePlayPlugin;

impl Plugin for PausePlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_pause);
    }
}

fn toggle_pause(
    sim_state: Res<State<SimState>>,
    mut next_state: ResMut<NextState<SimState>>,
    key_input: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindingsConfig>,
) {
    if key_input.just_pressed(key_bindings.time_controls.toggle_pause) {
        match sim_state.as_ref().get() {
            SimState::Stopped => next_state.set(SimState::Running),
            SimState::Running => next_state.set(SimState::Stopped),
            _ => next_state.set(SimState::Running)
        }
    }
}

struct ChangeTimeScalePlugin;

impl Plugin for ChangeTimeScalePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, modify_time_scale);
    }
}

fn modify_time_scale(
    mut time_options: ResMut<TimeScaleOptions>,
    key_input: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindingsConfig>,
) {
    if key_input.just_pressed(key_bindings.time_controls.faster) {
        time_options.multiplier += key_bindings.time_controls.scale_step;
    }
    if key_input.just_pressed(key_bindings.time_controls.slower) {
        time_options.multiplier -= key_bindings.time_controls.scale_step;
    }
    if key_input.just_pressed(key_bindings.time_controls.reset_speed) {
        time_options.reset();
    }
    if key_input.just_pressed(key_bindings.time_controls.toggle_real_time) {
        time_options.toggle_real_time();
    }
}
