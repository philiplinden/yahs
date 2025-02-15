use bevy::prelude::*;
use bevy_console::{reply, ConsoleCommand, AddConsoleCommand};
use clap::{Parser, Subcommand};
use yahs::prelude::*;

#[derive(Parser)]
#[command(name = "yahs")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the simulation
    Go(GoCommand),
    /// Stop the simulation
    Stop(StopCommand),
    /// Step the simulation forward
    Step(StepCommand),
}

/// Start the simulation
#[derive(Parser, ConsoleCommand)]
#[command(name = "go")]
pub struct GoCommand {}

/// Stop the simulation
#[derive(Parser, ConsoleCommand)]
#[command(name = "stop")] 
pub struct StopCommand {}

#[derive(Parser, ConsoleCommand)]
#[command(name = "step")]
pub struct StepCommand {
    #[arg(default_value = "0.1")]
    pub step_size: f32,
}

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

pub fn step_command(
    mut cmd: ConsoleCommand<StepCommand>,
    mut event_writer: EventWriter<StepPhysicsEvent>,
) {
    if let Some(Ok(args)) = cmd.take() {
        info!("Stepping simulation by {}", args.step_size);
        event_writer.send(StepPhysicsEvent(args.step_size));
        reply!(cmd, "Stepped simulation by {}", args.step_size);
    }
}

pub struct CliPlugin;

impl Plugin for CliPlugin {
    fn build(&self, app: &mut App) {
        app.add_console_command::<GoCommand, _>(go_command)
           .add_console_command::<StopCommand, _>(stop_command)
           .add_console_command::<StepCommand, _>(step_command);
    }
}
