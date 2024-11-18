use bevy::prelude::*;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KeyBindingsConfig>();
    }
}

#[derive(Resource)]
pub struct KeyBindingsConfig {
    pub camera_controls: CameraControls,
    pub debug_controls: DebugControls,
}

impl Default for KeyBindingsConfig {
    fn default() -> Self {
        Self {
            camera_controls: CameraControls::default(),
            debug_controls: DebugControls::default(),
        }
    }
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
    pub toggle_wireframe: KeyCode,
    pub toggle_inspector: KeyCode,
    pub toggle_perf_ui: KeyCode,
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
            toggle_wireframe: KeyCode::KeyW,
            toggle_inspector: KeyCode::KeyI,
            toggle_perf_ui: KeyCode::KeyP,
        }
    }
}
