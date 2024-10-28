use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

mod shell;
// Import other UI-related modules here
// mod config_view;
// mod flight_view;
// mod stats_view;

use shell::ShellPlugin;
// Use other UI-related plugins here
// use config_view::ConfigViewPlugin;
// use flight_view::FlightViewPlugin;
// use stats_view::StatsViewPlugin;

// Re-export ShellPlugin and other UI-related items if needed
pub use shell::Shell;

/// A plugin group that includes all interface-related plugins
pub struct InterfacePlugins;

impl PluginGroup for InterfacePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ShellPlugin)
            // Add other UI-related plugins here
            // .add(ConfigViewPlugin)
            // .add(FlightViewPlugin)
            // .add(StatsViewPlugin)
    }
}

/// Something to view in the monitor windows
pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

/// Something to view
pub trait UiPanel {
    /// Is the monitor enabled for this integration?
    fn is_enabled(&self, _ctx: &egui::Context) -> bool {
        true
    }

    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}
