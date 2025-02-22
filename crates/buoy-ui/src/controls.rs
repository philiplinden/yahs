use bevy::prelude::*;

use buoy_core::prelude::SimState;

pub fn plugin(app: &mut App) {
    app.init_resource::<KeyBindingsConfig>();
    app.add_plugins(PausePlayPlugin);
}

#[allow(dead_code)]
#[derive(Resource, Default)]
pub struct KeyBindingsConfig {
    pub debug_controls: DebugControls,
    pub time_controls: TimeControls,
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
}

// ============================ DEFAULT KEYBINDINGS ============================

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
