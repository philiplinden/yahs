use clap::Parser;
use bevy_console::ConsoleCommand;
use yahs_cli::{Cli, Commands, start_command, get_command, set_command};

fn main() {
    let cli = Cli::parse();
    // match &cli.command {
    //     Commands::Start(cmd) => start_command(ConsoleCommand::new(cmd.clone())),
    //     Commands::Get(cmd) => get_command(ConsoleCommand::new(cmd.clone())),
    //     Commands::Set(cmd) => set_command(ConsoleCommand::new(cmd.clone())),
    // }
}
