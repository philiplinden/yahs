mod assets;
mod simulator;
mod ui;

#[cfg(feature = "dev")]
mod dev_tools;

use bevy::{asset::AssetMetaCheck, prelude::*};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Splash,
    Loading,
    Running,
}

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        setup_pretty_logs();

        // Order new `AppStep` variants by adding them here:
        app.configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        );

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);

        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist)
                    // if this isn't set. This causes errors and even panics on
                    // web build on itch. See
                    // https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "🎈".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        );

        // Add other plugins.
        app.add_plugins((
            ui::InterfacePlugins,
            assets::AssetTrackingPlugin,
            assets::ConfigLoaderPlugin,
            simulator::SimulatorPlugins,
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}

fn setup_pretty_logs() {
    // look for the RUST_LOG env var or default to "info"
    let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned());
    std::env::set_var("RUST_LOG", rust_log);
    // initialize pretty print logger
    pretty_env_logger::init();
}

/// High-level groupings of systems for the app in the `Update` schedule. When
/// adding a new variant, make sure to order it in the `configure_sets` call
/// above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera3dBundle::default(),
        // Render all UI to this camera. Not strictly necessary since we only
        // use one camera, but if we don't use this component, our UI will
        // disappear as soon as we add another camera. This includes indirect
        // ways of adding cameras like using [ui node
        // outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        IsDefaultUiCamera,
    ));
}
