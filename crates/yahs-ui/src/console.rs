use bevy::{
    prelude::*,
    log::{self, LogPlugin},
};
use bevy_console::{ConsoleConfiguration, ConsoleOpen, ConsolePlugin, make_layer};
use yahs_cli::CliPlugin;

const OPEN_CONSOLE_BY_DEFAULT: bool = false;

pub struct DevConsolePlugin;

impl Plugin for DevConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ConsolePlugin,
            LogPlugin {
                level: log::Level::INFO,
                filter: "info,capture_bevy_logs=info".to_owned(),
                custom_layer: make_layer,
            },
            CliPlugin,
        ))
        .insert_resource(ConsoleConfiguration {
            top_pos: 0.0,
            left_pos: 0.0,
            height: 300.0,
            width: 1280.0,
            show_title_bar: false,
            ..Default::default()
        })
        .insert_resource(ConsoleOpen {
            open: OPEN_CONSOLE_BY_DEFAULT,
        });
    }
}
