use serde::Deserialize;
use log::{info, error};

use super::balloon::MaterialType;
use super::gas::GasSpecies;
use std::{fs, path::Path};

pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Config {
    // Read the contents of the configuration file as string
    match fs::read_to_string(path) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(contents) => parse(&contents),
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            error!("Could not read file!");
            Config::default()
        }
    }
}

pub fn parse(contents: &String) -> Config {
    // unpack the config TOML from string
    match toml::from_str(contents) {
        Ok(parsed) => {
            info!("Parsed config:\n{:}", contents);
            parsed
        },
        Err(_) => {
            error!("Could not parse config! Using defaults.");
            Config::default()
        }
    }
}

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct Config {
    pub environment: EnvConfig,
    pub balloon: BalloonConfig,
    pub bus: BusConfig,
}

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct EnvConfig {
    pub real_time: bool,
    pub tick_rate_hz: f32,
    pub max_elapsed_time_s: f32,
    pub initial_altitude_m: f32,
    pub initial_velocity_m_s: f32,
}

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct BalloonConfig {
    pub material: MaterialType,          // balloon material
    pub thickness_m: f32,                // thickness of balloon membrane
    pub barely_inflated_diameter_m: f32, // assuming balloon is a sphere, diameter of "unstressed" balloon membrane when filled
    pub lift_gas: GasConfig,
}

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct GasConfig {
    pub species: GasSpecies,
    pub mass_kg: f32,
}

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct BusConfig {
    pub body: BodyConfig,
    pub parachute: ParachuteConfig,
    // pub control: ControlConfig,
}

#[derive(Copy, Clone, Default, Deserialize, PartialEq)]
pub struct BodyConfig {
    pub mass_kg: f32,      // mass of all components less ballast material
    pub drag_area_m2: f32, // effective area used for drag calculations during freefall
    pub drag_coeff: f32,   // drag coefficient of the payload during freefall
}

#[derive(Copy, Clone, Default, Deserialize, PartialEq)]
pub struct ParachuteConfig {
    pub total_mass_kg: f32,     // mass of the parachute system (main + drogue)
    pub drogue_area_m2: f32,    // drogue parachute effective area used for drag calculations
    pub drogue_drag_coeff: f32, // drogue parachute drag coefficient
    pub main_area_m2: f32,      // main parachute effective area used for drag calculations
    pub main_drag_coeff: f32,   // main parachute drag coefficient when fully deployed
    pub deploy_force_n: f32,    // force needed for the drogue to deploy the main chute
    pub deploy_time_s: f32,     // how long the main chute stays in the partially open state
}
