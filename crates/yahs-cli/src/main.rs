use clap::Parser;
use yahs_cli::{Cli, Commands, StepCommand};
use yahs::prelude::*;
use bevy::prelude::*;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Go(_) => {
            println!("Starting simulation");
            // Initialize app with running state
            App::new()
                .add_plugins(SimulatorPlugins)
                .init_state::<SimState>()
                .add_systems(Startup, |mut next_state: ResMut<NextState<SimState>>| {
                    next_state.set(SimState::Running);
                })
                .run();
        }
        Commands::Stop(_) => {
            println!("Stopping simulation");
            App::new()
                .add_plugins(SimulatorPlugins)
                .init_state::<SimState>()
                .add_systems(Startup, |mut next_state: ResMut<NextState<SimState>>| {
                    next_state.set(SimState::Stopped);
                })
                .run();
        }
        Commands::Step(cmd) => {
            println!("Stepping simulation by {}", cmd.step_size);
            App::new()
                .add_plugins(SimulatorPlugins)
                .init_state::<SimState>()
                .add_event::<StepPhysicsEvent>()
                .add_systems(Startup, move |cmd: Res<StepCommand>, mut event_writer: EventWriter<StepPhysicsEvent>| {
                    event_writer.send(StepPhysicsEvent(cmd.step_size.clone()));
                })
                .run();
        }
    }
}
