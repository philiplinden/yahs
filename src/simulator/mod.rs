mod balloon;
mod config;
mod constants;
mod forces;
mod gas;
mod heat;
mod payload;

use log::{debug, error, info, warn};
use std::{
    fs::File,
    path::PathBuf,
    process::exit,
    sync::mpsc,
    sync::mpsc::{Receiver, Sender},
    sync::{Arc, Mutex},
    thread::JoinHandle,
    time::{Duration, Instant},
};

use balloon::{Balloon, Material};
use config::{parse_config, Config};
use forces::{net_force, projected_spherical_area};
use gas::{Atmosphere, GasVolume};

pub struct SimCommands {
    pub vent_flow_percentage: f32,
    pub dump_flow_percentage: f32,
}

#[derive(Default, Copy, Clone)]
pub struct SimOutput {
    pub time_s: f32,
    pub altitude: f32,
    pub ascent_rate: f32,
    pub acceleration: f32,
    // pub ballast_mass: f32,
    // pub lift_gas_mass: f32,
    // pub vent_pwm: f32,
    // pub dump_pwm: f32,
    // pub gross_lift: f32,
    // pub free_lift: f32,
    pub atmo_temp: f32,
    pub atmo_pres: f32,
}

pub struct Rate {
    cycle_time: Duration,
    end_of_last_sleep: Option<Instant>,
}

impl Rate {
    pub fn new(rate_hz: f32) -> Self {
        Self {
            cycle_time: Duration::from_secs_f32(1.0 / rate_hz),
            end_of_last_sleep: None,
        }
    }

    pub fn sleep(&mut self) {
        let now = Instant::now();

        let sleep_duration = match self.end_of_last_sleep {
            Some(v) => self
                .cycle_time
                .checked_sub(now.checked_duration_since(v).expect(
                    "Rate sleep experienced a last sleep with time ahead of the current time",
                ))
                .expect("Rate sleep detected a blown cycle"),
            None => self.cycle_time,
        };

        std::thread::sleep(sleep_duration);

        self.end_of_last_sleep = Some(Instant::now());
    }
}

pub struct AsyncSim {
    config: Config,
    sim_output: Arc<Mutex<SimOutput>>,
    outpath: PathBuf,
    command_sender: Option<Sender<SimCommands>>,
    /// keep track of
    run_handle: Option<JoinHandle<()>>,
}

impl AsyncSim {
    pub fn new(config_path: &PathBuf, outpath: PathBuf) -> Self {
        Self {
            config: parse_config(config_path),
            sim_output: Arc::new(Mutex::new(SimOutput::default())),
            outpath,
            command_sender: None,
            run_handle: None,
        }
    }

    pub fn get_sim_output(&self) -> SimOutput {
        *self.sim_output.lock().unwrap()
    }

    /// Start a thread to run the sim
    pub fn start(&mut self) {
        if self.run_handle.is_some() {
            panic!("Can't start again, sim already ran. Need to stop.")
        }
        let config = self.config.clone();
        let output = self.sim_output.clone();
        let outpath = self.outpath.clone();

        let (s, command_receiver) = mpsc::channel();
        self.command_sender = Some(s);

        debug!("Creating simulation handler...");
        self.run_handle = Some(std::thread::spawn(move || {
            debug!("Simulation handler created. Initializing run...");
            AsyncSim::run_sim(config, command_receiver, output, outpath)
        }));
    }

    fn run_sim(
        config: Config,
        command_channel: Receiver<SimCommands>,
        sim_output: Arc<Mutex<SimOutput>>,
        outpath: PathBuf,
    ) {
        let mut sim_state = initialize(&config);

        // configure simulation
        let physics_rate = config.environment.tick_rate_hz;
        let max_sim_time = config.environment.max_elapsed_time_s;
        let real_time = config.environment.real_time;
        let mut rate_sleeper = Rate::new(physics_rate);

        // set up data logger
        let mut writer = init_log_file(outpath);

        debug!("Simulation run initialized. Starting loop...");
        loop {
            if real_time {
                rate_sleeper.sleep();
            }
            sim_state = step(sim_state, &config);
            // Sync update all the fields
            {
                let mut output = sim_output.lock().unwrap();
                output.time_s = sim_state.time;
                output.altitude = sim_state.altitude;
                output.ascent_rate = sim_state.ascent_rate;
                output.acceleration = sim_state.acceleration;
                output.atmo_temp = sim_state.atmosphere.temperature();
                output.atmo_pres = sim_state.atmosphere.pressure();
                log_to_file(&output, &mut writer);
            }

            // Print log to terminal
            debug!(
                "[{:.3} s] | Atmosphere @ {:} m: {:} K, {:} Pa",
                sim_state.time,
                sim_state.altitude,
                sim_state.atmosphere.temperature(),
                sim_state.atmosphere.temperature()
            );
            info!(
                "[{:.3} s] | HAB @ {:.2} m, {:.3} m/s, {:.3} m/s^2 | {:.2} kg gas",
                sim_state.time,
                sim_state.altitude,
                sim_state.ascent_rate,
                sim_state.acceleration,
                sim_state.balloon.lift_gas.mass(),
            );
            // Stop if there is a problem
            if sim_state.altitude.is_nan()
                | sim_state.ascent_rate.is_nan()
                | sim_state.acceleration.is_nan()
            {
                error!("Something went wrong, a physical value is NaN!");
                exit(1);
            }
            // Run for a certain amount of sim time or to a certain altitude
            if sim_state.time >= max_sim_time {
                warn!("Simulation reached maximum time step. Stopping...");
                break;
            }
            if sim_state.altitude < 0.0 {
                error!("Simulation altitude cannot be below zero. Stopping...");
                break;
            }
        }
        exit(0);
    }
}

fn init_log_file(outpath: PathBuf) -> csv::Writer<File> {
    let mut writer = csv::Writer::from_path(outpath).unwrap();
    writer
        .write_record(&[
            "time_s",
            "altitude_m",
            "ascent_rate_m_s",
            "acceleration_m_s2",
            "atmo_temp_K",
            "atmo_pres_Pa",
        ])
        .unwrap();
    writer
}

fn log_to_file(sim_output: &SimOutput, writer: &mut csv::Writer<File>) {
    writer
        .write_record(&[
            sim_output.time_s.to_string(),
            sim_output.altitude.to_string(),
            sim_output.ascent_rate.to_string(),
            sim_output.acceleration.to_string(),
            sim_output.atmo_temp.to_string(),
            sim_output.atmo_pres.to_string(),
        ])
        .unwrap();
    writer.flush().unwrap();
}

pub struct SimInstant {
    pub time: f32,
    pub altitude: f32,
    pub ascent_rate: f32,
    pub acceleration: f32,
    pub atmosphere: Atmosphere,
    pub balloon: Balloon,
}

fn initialize(config: &Config) -> SimInstant {
    // create an initial time step based on the config
    let atmo = Atmosphere::new(config.environment.initial_altitude_m);
    let material = Material::new(config.balloon.material);
    let mut lift_gas = GasVolume::new(
        config.balloon.lift_gas.species,
        config.balloon.lift_gas.mass_kg,
    );
    lift_gas.update_from_ambient(atmo);
    SimInstant {
        time: 0.0,
        altitude: config.environment.initial_altitude_m,
        ascent_rate: config.environment.initial_velocity_m_s,
        acceleration: 0.0,
        atmosphere: atmo,
        balloon: Balloon::new(
            material,
            config.balloon.thickness_m,
            config.balloon.barely_inflated_diameter_m, // ballon diameter (m)
            lift_gas,
        ),
    }
}

pub fn step(input: SimInstant, config: &Config) -> SimInstant {
    // propagate the closed loop simulation forward by one time step
    let delta_t = 1.0 / config.environment.tick_rate_hz;
    let time = input.time + delta_t;
    let mut atmosphere = input.atmosphere;
    let mut balloon = input.balloon;
    // mass properties
    // let dump_mass_flow_rate = config.payload.control.dump_mass_flow_kg_s;
    // let ballast_mass =
    //     (input.ballast_mass - (input.dump_pwm * dump_mass_flow_rate)).max(0.0);
    // balloon
    //     .lift_gas
    //     .set_mass((balloon.lift_gas.mass() - input.vent_pwm * config.vent_mass_flow_rate).max(0.0));
    // let payload_dry_mass = input.bus.dry_mass;
    // let total_dry_mass = payload_dry_mass + ballast_mass;
    let total_dry_mass = config.payload.bus.dry_mass_kg;

    // switch drag conditions if the balloon has popped
    let projected_area: f32;
    let drag_coeff: f32;

    if balloon.intact {
        // balloon is intact
        balloon.stretch(atmosphere.pressure());
        projected_area = projected_spherical_area(balloon.lift_gas.volume());
        drag_coeff = balloon.drag_coeff;
    } else {
        // balloon has popped
        if input.altitude <= config.payload.parachute.open_altitude_m {
            // parachute open
            projected_area = config.payload.parachute.area_m2;
            drag_coeff = config.payload.parachute.drag_coeff;
        } else {
            // free fall, parachute not open
            projected_area = config.payload.bus.drag_area_m2;
            drag_coeff = config.payload.bus.drag_coeff;
        }
    }

    // heat transfer
    balloon.lift_gas.set_temperature(atmosphere.temperature());

    // calculate the net force
    let net_force = net_force(
        input.altitude,
        input.ascent_rate,
        atmosphere,
        balloon.lift_gas,
        projected_area,
        drag_coeff,
        total_dry_mass,
    );

    let acceleration = net_force / total_dry_mass;
    let ascent_rate = input.ascent_rate + acceleration * delta_t;
    let altitude = input.altitude + ascent_rate * delta_t;

    atmosphere.set_altitude(altitude);

    // derived outputs
    // let gross_lift = gross_lift(atmosphere, balloon.lift_gas);
    // let free_lift = free_lift(atmosphere, balloon.lift_gas, total_dry_mass);

    // // atmosphere stats
    // let atmo_temp = atmosphere.temperature();
    // let atmo_pres = atmosphere.pressure();

    SimInstant {
        time,
        altitude,
        ascent_rate,
        acceleration,
        atmosphere,
        balloon,
    }
}
