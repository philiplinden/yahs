mod cli;
mod simulator;

#[cfg(feature = "gui")]
mod gui;

fn main() {
    // look for the RUST_LOG env var or default to "info"
    let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned());
    std::env::set_var("RUST_LOG", rust_log);
    // initialize pretty print logger
    pretty_env_logger::init();
    // parse the commands, arguments, and options
    cli::parse_inputs();
}
