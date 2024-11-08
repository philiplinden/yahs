use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

use crate::simulator::{balloon::BalloonMaterial, gas::GasSpecies};
use crate::AppState;

#[derive(Resource, Asset, TypePath, Debug, Deserialize)]
pub struct PropertiesConfig {
    pub gases: Vec<GasSpecies>,
    pub materials: Vec<BalloonMaterial>,
}

#[derive(Resource)]
pub struct PropertiesConfigHandle(Handle<PropertiesConfig>);

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RonAssetPlugin::<PropertiesConfig>::new(&["ron"]),))
            .add_systems(Startup, setup_asset_loader)
            .add_systems(OnEnter(AppState::Loading), load_assets);
    }
}

fn setup_asset_loader(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(PropertiesConfigHandle(asset_server.load("properties.ron")));
}

fn load_assets(
    properties_handle: Res<PropertiesConfigHandle>,
    properties: Res<Assets<PropertiesConfig>>,
    mut commands: Commands,
    mut state: ResMut<NextState<AppState>>,
) {
    if let Some(properties_config) = properties.get(&properties_handle.0) {
        // Insert the loaded properties as a resource
        commands.insert_resource(PropertiesConfig {
            gases: properties_config.gases.clone(),
            materials: properties_config.materials.clone(),
        });
        // Transition to the Running state
        state.set(AppState::Running);
    }
}
