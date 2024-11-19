use bevy::{app::PluginGroupBuilder, prelude::*};
use iyes_perf_ui::prelude::*;

use crate::controls::KeyBindingsConfig;
use crate::simulator::SimState;

use super::*;

/// A plugin group that includes all interface-related plugins
pub struct InterfacePlugins;

impl PluginGroup for InterfacePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CoreUiPlugin)
            .add(PausePlayPlugin)
            .add(monitors::MonitorsPlugin)
    }
}

/// Base UI plugin. This sets up the base plugins that all other ui plugins
/// need. .Placeholder for now
pub struct CoreUiPlugin;

impl Plugin for CoreUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PerfUiPlugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
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
            _ => ()
        }
    }
}
