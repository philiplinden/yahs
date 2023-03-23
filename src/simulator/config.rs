use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::process::exit;
use toml;

use super::balloon::MaterialType;
use super::gas::GasSpecies;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub physics: PhysicsConfig,
    pub balloon: BalloonConfig,
    pub payload: PayloadConfig,
}

#[derive(Clone, Deserialize)]
pub struct PhysicsConfig {
    pub real_time: bool,
    pub tick_rate_hz: f32,
    pub max_elapsed_time_s: f32,
    pub initial_altitude_m: f32,
    pub initial_velocity_m_s: f32,
}

#[derive(Clone, Deserialize)]
pub struct BalloonConfig {
    pub material: MaterialType, // balloon material
    pub thickness_m: f32, // thickness of balloon membrane
    pub barely_inflated_diameter_m: f32, // assuming balloon is a sphere, diameter of "unstressed" balloon membrane when filled
    pub lift_gas: GasConfig,
}

#[derive(Clone, Deserialize)]
pub struct GasConfig {
    pub species: GasSpecies,
    pub mass_kg: f32,
}

#[derive(Clone, Deserialize)]
pub struct PayloadConfig{
    pub bus: BusConfig,
    pub parachute: ParachuteConfig,
    // pub control: ControlConfig,
}

#[derive(Clone, Deserialize)]
pub struct BusConfig {
    pub dry_mass_kg: f32, // mass of all components less ballast material
    pub drag_area_m2: f32, // effective area used for drag calculations during freefall
    pub drag_coeff: f32, // drag coefficient of the payload during freefall
}

#[derive(Clone, Deserialize)]
pub struct ParachuteConfig {
    pub area_m2: f32, // effective area used for drag calculations
    pub drag_coeff: f32, // drag coefficient when fully deployed
    pub open_altitude_m: f32, // altitude when the parachute fully deploys
}

// #[derive(Clone, Deserialize)]
// pub struct ControlConfig {
//     controller_path: PathBuf, 
//     pub vent_valve_mass_flow_kg_s: f32,
//     pub dump_mass_flow_kg_s: f32,
//     ballast_mass_kg: f32,
// }

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
            exit(1);
        }
    };

    // unpack the config TOML from string
    let cfg: Config = toml::from_str(&contents).unwrap();
    return cfg
}