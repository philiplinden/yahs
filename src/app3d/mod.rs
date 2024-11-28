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

use crate::simulator::SimState;

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
            ForceArrowsPlugin,
            MonitorsPlugin,
            #[cfg(feature = "dev")]
            DevToolsPlugin,
        ));
    }
}

pub struct PausePlayPlugin;

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
