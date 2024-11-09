use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

use crate::{
    simulator::{balloon::BalloonMaterial, gas::GasSpecies},
    AppState,
};

/// Configuration for the properties of gases and materials.
#[derive(Asset, Debug, Deserialize, TypePath)]
pub struct PropertiesConfig {
    pub gases: Vec<GasSpecies>,
    pub materials: Vec<BalloonMaterial>,
}

impl Default for PropertiesConfig {
    fn default() -> Self {
        Self { gases: vec![], materials: vec![] }
    }
}

/// Plugin for loading configuration.
pub struct ConfigLoaderPlugin;

impl Plugin for ConfigLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<PropertiesConfig>::new(&["ron"]))
            .insert_resource(PropertiesConfig::default())
            .add_systems(
                Startup,
                load_configs,
            );
    }
}

/// Loads the configuration and transitions to the Running state.
fn load_configs(asset_server: Res<AssetServer>, mut commands: Commands) {
    info!("Setting up configuration loader");
    let handle = asset_server.load("configs/properties.ron");
    commands.insert_resource(handle.clone());
    info!("Configuration loader setup complete");
}
