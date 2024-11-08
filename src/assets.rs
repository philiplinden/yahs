use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;
use std::collections::VecDeque;

use crate::simulator::{balloon::BalloonMaterial, gas::GasSpecies};
use crate::AppState;

/// Configuration for the properties of gases and materials.
#[derive(Resource, Asset, Debug, Deserialize, Reflect)]
#[reflect(Resource)]
pub struct PropertiesConfig {
    pub gases: Vec<GasSpecies>,
    pub materials: Vec<BalloonMaterial>,
}

impl Default for PropertiesConfig {
    fn default() -> Self {
        Self { gases: vec![], materials: vec![] }
    }
}

/// Asset handle for the properties configuration asset.
#[derive(Resource)]
pub struct PropertiesConfigHandle(Handle<PropertiesConfig>);

/// Plugin for loading configuration.
pub struct ConfigLoaderPlugin;

impl Plugin for ConfigLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<PropertiesConfig>::new(&["ron"]))
            .insert_resource(PropertiesConfig::default())
            .add_systems(
                Startup,
                load_configs,
            )
            .add_systems(
                Update,
                check_configs_loaded.run_if(in_state(AppState::Loading)),
            );
    }
}

/// Loads the configuration and transitions to the Running state.
///
/// This function now includes the logic previously in `setup_config_loader`.
fn load_configs(asset_server: Res<AssetServer>, mut commands: Commands) {
    info!("Setting up configuration loader");
    let handle = asset_server.load("configs/properties.ron");
    commands.insert_resource(PropertiesConfigHandle(handle.clone()));
    info!("Configuration loader setup complete");
}

/// Checks if the configuration is loaded and transitions to Running state.
fn check_configs_loaded(
    properties_handle: Res<PropertiesConfigHandle>,
    properties: Res<Assets<PropertiesConfig>>,
    mut commands: Commands,
    mut state: ResMut<NextState<AppState>>,
) {
    if let Some(properties_config) = properties.get(&properties_handle.0) {
        info!("Configuration loaded successfully");
        commands.insert_resource(PropertiesConfig {
            gases: properties_config.gases.clone(),
            materials: properties_config.materials.clone(),
        });
        state.set(AppState::Running);
        info!("Transitioning to Running state");
    } else {
        warn!("Configuration not yet loaded");
    }
}

/// A high-level way to load collections of asset handles as resources.
pub struct AssetTrackingPlugin;

impl Plugin for AssetTrackingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ResourceHandles>()
            .add_systems(PreUpdate, load_resource_assets);
    }
}

pub trait LoadResource {
    /// Loads the [`Resource`] as an [`Asset`]. When all asset dependencies are loaded,
    /// it inserts the resource into the world.
    fn load_resource<T: Resource + Asset + Clone + FromWorld>(&mut self) -> &mut Self;
}

impl LoadResource for App {
    fn load_resource<T: Resource + Asset + Clone + FromWorld>(&mut self) -> &mut Self {
        self.init_asset::<T>();
        let world = self.world_mut();
        let value = T::from_world(world);
        let assets = world.resource::<AssetServer>();
        let handle = assets.add(value);
        let mut handles = world.resource_mut::<ResourceHandles>();
        handles
            .waiting
            .push_back((handle.untyped(), |world, handle| {
                let assets = world.resource::<Assets<T>>();
                if let Some(value) = assets.get(handle.id().typed::<T>()) {
                    world.insert_resource(value.clone());
                }
            }));
        self
    }
}

type InsertLoadedResource = fn(&mut World, &UntypedHandle);

#[derive(Resource, Default)]
struct ResourceHandles {
    // Queue for waiting assets to be loaded
    waiting: VecDeque<(UntypedHandle, InsertLoadedResource)>,
    finished: Vec<UntypedHandle>,
}

fn load_resource_assets(world: &mut World) {
    world.resource_scope(|world, mut resource_handles: Mut<ResourceHandles>| {
        world.resource_scope(|world, assets: Mut<AssetServer>| {
            for _ in 0..resource_handles.waiting.len() {
                let (handle, insert_fn) = resource_handles.waiting.pop_front().unwrap();
                if assets.is_loaded_with_dependencies(&handle) {
                    insert_fn(world, &handle);
                    resource_handles.finished.push(handle);
                } else {
                    resource_handles.waiting.push_back((handle, insert_fn));
                }
            }
        });
    });
}
