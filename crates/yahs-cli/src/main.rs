use bevy::{
    prelude::*,
    window::{WindowPlugin, ExitCondition},
};
use bevy_ratatui::RatatuiPlugins;
use yahs::prelude::SimulatorPlugins;
use yahs_cli::TuiPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: None,
                close_when_requested: false,
                exit_condition: ExitCondition::DontExit, // exit in controls.rs
            }),
            RatatuiPlugins::default(),
            SimulatorPlugins,
            TuiPlugin,
        ))
        .run();
}
