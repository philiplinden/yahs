use bevy::prelude::*;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KeyBindingsConfig>();
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
    pub cycle_target: KeyCode,
    pub modifier_pan: KeyCode,
    pub button_pan: MouseButton,
    pub button_orbit: MouseButton,
    pub zoom_step: f32,
    pub max_fov: f32,
    pub min_fov: f32,
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
    pub scale_step: f32,
}

// ============================ DEFAULT KEYBINDINGS ============================

/// Defaults follow Blender conventions
impl Default for CameraControls {
    fn default() -> Self {
        Self {
            cycle_target: KeyCode::Tab,
            modifier_pan: KeyCode::ShiftLeft,
            button_pan: MouseButton::Middle,
            button_orbit: MouseButton::Middle,
            zoom_step: 0.01,
            max_fov: 1.0,
            min_fov: 0.01,
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
            scale_step: 0.1,
        }
    }
}
