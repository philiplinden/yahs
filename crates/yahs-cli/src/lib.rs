use bevy_console::{reply, ConsoleCommand};
use clap::{Parser, Subcommand};
use tracing::info;

#[derive(Parser)]
#[command(name = "yahs")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start(StartCommand),
    Get(GetCommand),
    Set(SetCommand),
}

/// Start a new simulation process
#[derive(Parser, ConsoleCommand)]
#[command(name = "start")]
pub struct StartCommand {
    #[arg(short, long, value_name = "TOML", default_value = "config/default.toml")]
    pub config: String,

    #[arg(short, long, value_name = "CSV", default_value = "./out.csv")]
    pub outpath: String,
}

/// Inspect a physics parameter
#[derive(Parser, ConsoleCommand)]
#[command(name = "get")]
pub struct GetCommand {
    /// Parameter to inspect
    pub param: String,
}

/// Modify a physics parameter
#[derive(Parser, ConsoleCommand)]
#[command(name = "set")]
pub struct SetCommand {
    /// Parameter to modify
    pub param: String,
    /// New value
    pub value: String,
}

pub fn start_command(mut cmd: ConsoleCommand<StartCommand>) {
    if let Some(Ok(args)) = cmd.take() {
        info!("Starting simulation with config: {:?}", args.config);
        info!("Output file: {:?}", args.outpath);
        reply!(cmd, "Started simulation");
    }
}

// Add handlers for get and set commands
pub fn get_command(mut cmd: ConsoleCommand<GetCommand>) {
    if let Some(Ok(args)) = cmd.take() {
        info!("Getting parameter: {:?}", args.param);
        reply!(cmd, "Parameter value retrieved");
    }
}

pub fn set_command(mut cmd: ConsoleCommand<SetCommand>) {
    if let Some(Ok(args)) = cmd.take() {
        info!("Setting parameter {} to {}", args.param, args.value);
        reply!(cmd, "Parameter updated");
    }
}
