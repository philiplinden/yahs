use log::info;
use std::path::PathBuf;
use toml::Value;

use crate::simulator::{AsyncSim, Rate};

pub fn start_sim(sim_config: &PathBuf, outfile: &PathBuf) {
    let config = std::fs::read_to_string(sim_config)
        .unwrap()
        .as_str()
        .parse::<Value>()
        .unwrap();

    info!("Setting up sim with the following config: \n{}", config);

    // initialize the simulation
    let mut sim = AsyncSim::new(config, outfile.clone());
    let mut rate_sleeper = Rate::new(1.0);

    // start the sim
    sim.start();
    loop {
        sim.get_sim_output();
        rate_sleeper.sleep();
    }
}
