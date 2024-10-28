use bevy::prelude::*;
use serde::Deserialize;
use log::{info, error};

use crate::simulation::balloon::MaterialType;
use crate::simulation::gas::GasSpecies;
use std::{fs, path::Path};

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        // Initialize default configurations
        app.init_resource::<EnvConfig>()
           .init_resource::<BalloonConfig>()
           .init_resource::<GasConfig>()
           .init_resource::<BusConfig>()
           .init_resource::<BodyConfig>()
           .init_resource::<ParachuteConfig>();

        // Add a system to load configuration from file
        // app.add_systems(Startup, load_config);
    }
}

#[derive(Clone, Default, Deserialize, PartialEq, Resource)]
pub struct EnvConfig {
    pub real_time: bool,
    pub tick_rate_hz: f32,
    pub max_elapsed_time_s: f32,
    pub initial_altitude_m: f32,
    pub initial_velocity_m_s: f32,
}

#[derive(Clone, Default, Deserialize, PartialEq, Resource)]
pub struct BalloonConfig {
    /// Balloon material type
    pub material: MaterialType,
    /// Thickness of balloon membrane in meters
    pub thickness_m: f32,
    /// Diameter of "unstressed" balloon membrane when filled, assuming balloon is a sphere, in meters
    pub barely_inflated_diameter_m: f32,
    /// Configuration for the lift gas
    pub lift_gas: GasConfig,
}

#[derive(Clone, Default, Deserialize, PartialEq, Resource)]
pub struct GasConfig {
    /// Species of the gas
    pub species: GasSpecies,
    /// Mass of the gas in kilograms
    pub mass_kg: f32,
}

#[derive(Clone, Default, Deserialize, PartialEq, Resource)]
pub struct BusConfig {
    /// Configuration for the body of the bus
    pub body: BodyConfig,
    /// Configuration for the parachute system
    pub parachute: ParachuteConfig,
}

#[derive(Copy, Clone, Default, Deserialize, PartialEq, Resource)]
pub struct BodyConfig {
    /// Mass of all components less ballast material, in kilograms
    pub mass_kg: f32,
    /// Effective area used for drag calculations during freefall, in square meters
    pub drag_area_m2: f32,
    /// Drag coefficient of the payload during freefall
    pub drag_coeff: f32,
}

#[derive(Copy, Clone, Default, Deserialize, PartialEq, Resource)]
pub struct ParachuteConfig {
    /// Mass of the parachute system (main + drogue), in kilograms
    pub total_mass_kg: f32,
    /// Drogue parachute effective area used for drag calculations, in square meters
    pub drogue_area_m2: f32,
    /// Drogue parachute drag coefficient
    pub drogue_drag_coeff: f32,
    /// Main parachute effective area used for drag calculations, in square meters
    pub main_area_m2: f32,
    /// Main parachute drag coefficient when fully deployed
    pub main_drag_coeff: f32,
    /// Force needed for the drogue to deploy the main chute, in Newtons
    pub deploy_force_n: f32,
    /// Duration the main chute stays in the partially open state, in seconds
    pub deploy_time_s: f32,
}
