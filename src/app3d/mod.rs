mod camera;
pub mod controls;
mod gizmos;
mod scene;
mod dev_tools;
mod monitors;

use camera::CameraPlugin;
use controls::{ControlsPlugin, KeyBindingsConfig};
use gizmos::ForceArrowsPlugin;
use scene::ScenePlugin;
use dev_tools::DevToolsPlugin;
use monitors::MonitorsPlugin;

use bevy::{app::{PluginGroup, PluginGroupBuilder}, prelude::*};

use crate::simulator::{SimState, time::TimeScaleOptions};

pub struct App3dPlugins;

impl PluginGroup for App3dPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(InterfacePlugin)
            .add(ControlsPlugin)
            .add(ScenePlugin)
            .add(CameraPlugin)
    }
}

/// A plugin group that includes all interface-related plugins
pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PausePlayPlugin,
            ChangeTimeScalePlugin,
            ForceArrowsPlugin,
            MonitorsPlugin,
            #[cfg(feature = "dev")]
            DevToolsPlugin,
        ));
    }
}

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
        time_options.multiplier = 1.0;
    }
}
