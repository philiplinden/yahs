use serde::Deserialize;

use super::balloon::MaterialType;
use super::gas::GasSpecies;
use std::{fs, path::PathBuf};

pub fn parse_config(filepath: &PathBuf) -> Config {
    // Read the contents of the configuration file as string
    let contents = match fs::read_to_string(filepath) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file `{}`", filepath.to_string_lossy());
            // Exit the program with exit code `1`.
            std::process::exit(1);
        }
    };

    // unpack the config TOML from string
    return toml::from_str(&contents).unwrap();
}

#[derive(Clone, Deserialize)]
pub struct Config {
    pub environment: EnvConfig,
    pub balloon: BalloonConfig,
    pub bus: BusConfig,
}

#[derive(Clone, Deserialize)]
pub struct EnvConfig {
    pub real_time: bool,
    pub tick_rate_hz: f32,
    pub max_elapsed_time_s: f32,
    pub initial_altitude_m: f32,
    pub initial_velocity_m_s: f32,
}

#[derive(Clone, Deserialize)]
pub struct BalloonConfig {
    pub material: MaterialType,          // balloon material
    pub thickness_m: f32,                // thickness of balloon membrane
    pub barely_inflated_diameter_m: f32, // assuming balloon is a sphere, diameter of "unstressed" balloon membrane when filled
    pub lift_gas: GasConfig,
}

#[derive(Clone, Deserialize)]
pub struct GasConfig {
    pub species: GasSpecies,
    pub mass_kg: f32,
}

#[derive(Clone, Deserialize)]
pub struct BusConfig {
    pub body: BodyConfig,
    pub parachute: ParachuteConfig,
    // pub control: ControlConfig,
}

#[derive(Copy, Clone, Deserialize)]
pub struct BodyConfig {
    pub mass_kg: f32,      // mass of all components less ballast material
    pub drag_area_m2: f32, // effective area used for drag calculations during freefall
    pub drag_coeff: f32,   // drag coefficient of the payload during freefall
}

#[derive(Copy, Clone, Deserialize)]
pub struct ParachuteConfig {
    pub total_mass_kg: f32,     // mass of the parachute system (main + drogue)
    pub drogue_area_m2: f32,    // drogue parachute effective area used for drag calculations
    pub drogue_drag_coeff: f32, // drogue parachute drag coefficient
    pub main_area_m2: f32,      // main parachute effective area used for drag calculations
    pub main_drag_coeff: f32,   // main parachute drag coefficient when fully deployed
    pub deploy_force_n: f32,    // force needed for the drogue to deploy the main chute
    pub deploy_time_s: f32,     // how long the main chute stays in the partially open state
}
