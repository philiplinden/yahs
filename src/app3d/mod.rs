mod scene;
mod ui;

// Re-export the plugins so they can be added to the app with `app.add_plugins`.
pub use scene::ScenePlugin;
pub use ui::InterfacePlugins;
