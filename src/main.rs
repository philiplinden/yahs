mod cli;
mod simulator;

#[cfg(feature = "gui")]
mod gui;

fn main() {
    // initialize pretty print logger
    pretty_env_logger::init();
    // parse the commands, arguments, and options
    cli::parse_inputs();
}
