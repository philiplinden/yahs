use clap::Parser;
use yahs_cli::Cli;

fn main() {
    let _cli = Cli::parse();
    // match &cli.command {
    //     Commands::Start(cmd) => start_command(ConsoleCommand::new(cmd.clone())),
    //     Commands::Get(cmd) => get_command(ConsoleCommand::new(cmd.clone())),
    //     Commands::Set(cmd) => set_command(ConsoleCommand::new(cmd.clone())),
    // }
}
