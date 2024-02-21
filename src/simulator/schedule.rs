use log::{debug, error, info, warn};
use std::{
    path::PathBuf,
    process::exit,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::JoinHandle,
    time::{Duration, Instant},
};

use crate::simulator::{
    balloon::{Balloon, Material},
    bus::{Body, ParachuteSystem},
    config::{parse_config, Config},
    forces,
    gas::{Atmosphere, GasVolume},
    io::{SimCommands, SimOutput},
    SolidBody,
};

/// The simulation state at a given instant
pub struct SimInstant {
    pub time: f32,
    pub altitude: f32,
    pub ascent_rate: f32,
    pub acceleration: f32,
    pub atmosphere: Atmosphere,
    pub balloon: Balloon,
    pub body: Body,
    pub parachute: ParachuteSystem,
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
    let body = Body::new(config.bus.body);
    let parachute =
        ParachuteSystem::new(config.bus.parachute, 1.0 / config.environment.tick_rate_hz);

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
        body,
        parachute,
    }
}

pub fn step(input: SimInstant, config: &Config) -> SimInstant {
    // propagate the closed loop simulation forward by one time step
    let delta_t = 1.0 / config.environment.tick_rate_hz;
    let time = input.time + delta_t;
    let mut atmosphere = input.atmosphere;
    let mut balloon = input.balloon;
    let body = input.body;
    let mut parachute = input.parachute;

    if balloon.intact {
        // balloon is intact
        balloon.stretch(atmosphere.pressure());
    } else {
        parachute.deploy(atmosphere, input.ascent_rate);
    };
    let total_dry_mass = body.total_mass() + parachute.total_mass();
    let weight_force = forces::weight(input.altitude, total_dry_mass);
    let buoyancy_force = forces::buoyancy(input.altitude, atmosphere, balloon.lift_gas);

    let total_drag_force = forces::drag(atmosphere, input.ascent_rate, balloon)
        + forces::drag(atmosphere, input.ascent_rate, body)
        + forces::drag(atmosphere, input.ascent_rate, parachute.main)
        + forces::drag(atmosphere, input.ascent_rate, parachute.drogue);
    debug!(
        "weight: {:?} buoyancy: {:?} drag: {:?}",
        weight_force, buoyancy_force, total_drag_force
    );

    // calculate the net force
    let net_force = weight_force + buoyancy_force + total_drag_force;
    let acceleration = net_force / total_dry_mass;
    let ascent_rate = input.ascent_rate + acceleration * delta_t;
    let altitude = input.altitude + ascent_rate * delta_t;

    atmosphere.set_altitude(altitude);

    SimInstant {
        time,
        altitude,
        ascent_rate,
        acceleration,
        atmosphere,
        balloon,
        body,
        parachute,
    }
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
        _command_channel: Receiver<SimCommands>,
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
        // let mut writer = init_log_file(outpath);

        debug!("Simulation run initialized. Starting loop...");
        loop {
            if real_time {
                rate_sleeper.sleep();
            }
            sim_state = step(sim_state, &config);

            //log output

            // Print log to terminal
            debug!(
                "[{:.3} s] | Atmosphere @ {:} m: {:} K, {:} Pa",
                sim_state.time,
                sim_state.altitude,
                sim_state.atmosphere.temperature(),
                sim_state.atmosphere.temperature()
            );
            info!(
                "[{:.3} s] | HAB @ {:.2} m, {:.3} m/s, {:.3} m/s^2 | {:.2} m radius, {:.2} Pa stress, {:.2} % strain",
                sim_state.time,
                sim_state.altitude,
                sim_state.ascent_rate,
                sim_state.acceleration,
                sim_state.balloon.radius(),
                sim_state.balloon.stress(),
                sim_state.balloon.strain() * 100.0,
            );
            // Stop if there is a problem
            if sim_state.altitude.is_nan()
                | sim_state.ascent_rate.is_nan()
                | sim_state.acceleration.is_nan()
            {
                Self::terminate(1, format!("Something went wrong, a physical value is NaN!"));
            }
            // Run for a certain amount of sim time or to a certain altitude
            if sim_state.time >= max_sim_time {
                Self::terminate(
                    0,
                    format!("Reached maximum time step ({:?} s)", sim_state.time),
                );
            }
            if sim_state.altitude < 0.0 {
                Self::terminate(0, format!("Altitude at or below zero."));
            }
        }
    }
    fn terminate(code: i32, reason: String) {
        if code > 0 {
            error!(
                "Simulation terminated abnormally with code {:?}. Reason: {:?}",
                code, reason
            );
        } else {
            warn!("Simulation terminated normally. Reason: {:?}", reason);
        }
        exit(code);
    }
}
