use bevy::prelude::*;
use bevy_console::{reply, ConsoleCommand, AddConsoleCommand};
use clap::{Parser, Subcommand};
use tracing::info;
use yahs::prelude::*;

#[derive(Parser)]
#[command(name = "yahs")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Go(GoCommand),
    Stop(StopCommand),
}

/// Start the simulation
#[derive(Parser, ConsoleCommand)]
#[command(name = "go")]
pub struct GoCommand {}

/// Stop the simulation
#[derive(Parser, ConsoleCommand)]
#[command(name = "stop")] 
pub struct StopCommand {}

pub fn go_command(
    mut cmd: ConsoleCommand<GoCommand>,
    mut next_state: ResMut<NextState<SimState>>,
) {
    if let Some(Ok(_)) = cmd.take() {
        info!("Starting simulation");
        next_state.set(SimState::Running);
        reply!(cmd, "Simulation started");
    }
}

pub fn stop_command(
    mut cmd: ConsoleCommand<StopCommand>,
    mut next_state: ResMut<NextState<SimState>>,
) {
    if let Some(Ok(_)) = cmd.take() {
        info!("Stopping simulation");
        next_state.set(SimState::Stopped);
        reply!(cmd, "Simulation stopped");
    }
}

pub struct CliPlugin;

impl Plugin for CliPlugin {
    fn build(&self, app: &mut App) {
        app.add_console_command::<GoCommand, _>(go_command)
           .add_console_command::<StopCommand, _>(stop_command);
    }
}
