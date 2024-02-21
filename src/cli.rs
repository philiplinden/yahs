use std::path::PathBuf;

use clap::{Parser, Subcommand};
use log::error;

use crate::simulator::{AsyncSim, Rate};

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a new simulation process
    ///
    /// Configure an asynchronous physics simulation in the background. This
    /// simulation runs on the MFC with flight software code running in the
    /// loop and logs the simulation output to a CSV file.
    Start {
        /// Sets a custom simulation config file
        #[clap(
            short,
            long,
            value_name = "TOML",
            default_value = "config/default.toml"
        )]
        config: PathBuf,

        /// Sets a custom output file
        #[clap(short, long, value_name = "CSV", default_value = "./out.csv")]
        outpath: PathBuf,
    },

    /// Inspect a physics parameter in an existing simulation
    Get {
        /// Parameter to be inspect
        param: String,
    },

    /// Modify a physics parameter in an existing simulation
    Set {
        /// Parameter to be modified
        param: String,
        /// New value to set
        value: String,
    },

    /// Open a graphical user interface instead of the terminal interface
    Gui,
}

pub fn parse_inputs() {
    // parse CLI input args and options
    let cli = Cli::parse();
    match &cli.command {
        Commands::Start { config, outpath } => {
            start_sim(config, outpath);
        }
        Commands::Gui => {
            #[cfg(feature = "gui")]
            start_gui();

            #[cfg(not(feature = "gui"))]
            error!("GUI feature not enabled. Reinstall with `--features gui`")
        }
        _ => {
            error!("Command not implemented yet!")
        }
    }
}

pub fn start_sim(config: &PathBuf, outpath: &PathBuf) {
    // initialize the simulation
    let mut sim = AsyncSim::new(config, outpath.clone());
    let mut rate_sleeper = Rate::new(1.0);

    // start the sim
    sim.start();
    loop {
        sim.get_sim_output();
        rate_sleeper.sleep();
    }
}

#[cfg(feature = "gui")]
pub fn start_gui() {
    use crate::gui;
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Mission Control",
        native_options,
        Box::new(|cc| Box::new(gui::Shell::new(cc))),
    );
}
