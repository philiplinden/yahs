mod balloon;
mod physics;
mod gas;
mod materials;
mod constants;
mod tools;

pub mod simulate;

use log::{debug, error, info, warn};
use std::{
    fs::File,
    path::PathBuf,
    sync::mpsc,
    sync::mpsc::{Receiver, Sender},
    sync::{Arc, Mutex},
    thread::JoinHandle,
    time::{Duration, Instant},
};
use toml;

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
    pub ballast_mass: f32,
    pub lift_gas_mass: f32,
    pub vent_pwm: f32,
    pub dump_pwm: f32,
    pub gross_lift: f32,
    pub free_lift: f32,
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
    config: toml::Value,
    sim_output: Arc<Mutex<SimOutput>>,
    outpath: PathBuf,
    command_sender: Option<Sender<SimCommands>>,
    /// keep track of
    run_handle: Option<JoinHandle<()>>,
}

impl AsyncSim {
    pub fn new(config: toml::Value, outpath: PathBuf) -> Self {
        Self {
            config,
            sim_output: Arc::new(Mutex::new(SimOutput::default())),
            outpath: outpath,
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
        config: toml::Value,
        command_channel: Receiver<SimCommands>,
        sim_output: Arc<Mutex<SimOutput>>,
        outpath: PathBuf,
    ) {
        let (mut step_input, step_config) = simulate::init(&config);

        let mut current_vent_flow_percentage = 0.0;
        let mut current_dump_flow_percentage = 0.0;

        // configure simulation
        let physics_rate = tools::read_as_f32(&config, "physics_rate_hz");
        let max_sim_time = tools::read_as_f32(&config, "max_sim_time_s");
        let real_time = tools::read_as_bool(&config, "real_time");
        let mut rate_sleeper = Rate::new(physics_rate);

        // set up data logger
        let mut writer = init_log_file(outpath);

        debug!("Simulation run initialized. Starting loop...");
        loop {
            if real_time {
                rate_sleeper.sleep();
            }
            if let Ok(new_flow_percentages) = command_channel.try_recv() {
                current_vent_flow_percentage = new_flow_percentages.vent_flow_percentage;
                current_dump_flow_percentage = new_flow_percentages.dump_flow_percentage;
            }

            step_input.vent_pwm = current_vent_flow_percentage;
            step_input.dump_pwm = current_dump_flow_percentage;
            step_input = simulate::step(step_input, &step_config);
            // Sync update all the fields
            {
                let mut output = sim_output.lock().unwrap();
                output.time_s = step_input.time;
                output.altitude = step_input.altitude;
                output.ascent_rate = step_input.ascent_rate;
                output.acceleration = step_input.acceleration;
                output.lift_gas_mass = step_input.balloon.lift_gas.mass();
                output.ballast_mass = step_input.ballast_mass;
                output.vent_pwm = step_input.vent_pwm;
                output.dump_pwm = step_input.dump_pwm;
                output.atmo_temp = step_input.atmo_temp;
                output.atmo_pres = step_input.atmo_pres;
                log_to_file(&output, &mut writer);
            }
            
            // Print log to terminal
            debug!(
                "[{:.3} s] | Atmosphere @ {:} m: {:} K, {:} Pa",
                step_input.time, step_input.altitude, step_input.atmo_temp, step_input.atmo_pres
            );
            info!(
                "[{:.3} s] | HAB @ {:.2} m, {:.3} m/s, {:.3} m/s^2 | {:.2} kg gas, {:.2} kg ballast",
                step_input.time,
                step_input.altitude,
                step_input.ascent_rate,
                step_input.acceleration,
                step_input.balloon.lift_gas.mass(),
                step_input.ballast_mass
            );
            // Run for a certain amount of sim time or to a certain altitude
            if step_input.time >= max_sim_time {
                warn!("Simulation reached maximum time step. Stopping...");
                break;
            }
            if step_input.altitude < 0.0 {
                error!("Simulation altitude cannot be below zero. Stopping...");
                break;
            }
        }
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
            "lift_gas_mass_kg",
            "ballast_mass_kg",
            "vent_pwm",
            "dump_pwm",
            "gross_lift_N",
            "free_lift_N",
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
            sim_output.lift_gas_mass.to_string(),
            sim_output.ballast_mass.to_string(),
            sim_output.vent_pwm.to_string(),
            sim_output.dump_pwm.to_string(),
            sim_output.gross_lift.to_string(),
            sim_output.free_lift.to_string(),
            sim_output.atmo_temp.to_string(),
            sim_output.atmo_pres.to_string(),
        ])
        .unwrap();
    writer.flush().unwrap();
}