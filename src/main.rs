mod ui;
mod simulator;
mod assets;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Running,
}

fn main() {
    setup_pretty_logs();
    App::new()
        .init_state::<AppState>()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "🎈".to_string(),
                    ..default()
                }),
                ..default()
            }),
            ui::InterfacePlugins,
            simulator::SimulatorPlugins,
            assets::AssetLoaderPlugin,
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
