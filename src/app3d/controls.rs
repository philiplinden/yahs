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
    pub modifier_pan: Option<KeyCode>,
    pub button_pan: MouseButton,
    pub button_orbit: MouseButton,
    pub toggle_zoom_direction: KeyCode,
}

#[derive(Reflect)]
pub struct DebugControls {
    pub toggle_inspector: KeyCode,
    pub toggle_wireframe: KeyCode,
    pub toggle_physics_debug: KeyCode,
    pub toggle_perf_ui: KeyCode,
    pub toggle_anything_else: KeyCode,
}

#[derive(Reflect)]
pub struct TimeControls {
    pub toggle_pause: KeyCode,
}

// ============================ DEFAULT KEYBINDINGS ============================

/// Defaults follow Blender conventions
impl Default for CameraControls {
    fn default() -> Self {
        Self {
            modifier_pan: Some(KeyCode::ShiftLeft),
            button_pan: MouseButton::Middle,
            button_orbit: MouseButton::Middle,
            toggle_zoom_direction: KeyCode::KeyZ,
        }
    }
}

impl Default for DebugControls {
    fn default() -> Self {
        Self {
            toggle_wireframe: KeyCode::F1,
            toggle_inspector: KeyCode::F2,
            toggle_physics_debug: KeyCode::F3,
            toggle_perf_ui: KeyCode::F4,
            toggle_anything_else: KeyCode::F5,
        }
    }
}

impl Default for TimeControls {
    fn default() -> Self {
        Self {
            toggle_pause: KeyCode::Space,
        }
    }
}
