mod gui;
mod config;
mod simulator;

use bevy::prelude::*;

fn main() {
    setup_pretty_logs();
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "🎈".to_string(),
                    ..default()
                }),
                ..default()
            }),
            gui::InterfacePlugins,
            config::ConfigPlugin,
        ))
        .run();
}

fn setup_pretty_logs() {
    // look for the RUST_LOG env var or default to "info"
    let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned());
    std::env::set_var("RUST_LOG", rust_log);
    // initialize pretty print logger
    pretty_env_logger::init();
}
