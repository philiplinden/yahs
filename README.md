# yet another HAB simulator

A high altitude balloon flight simulator based on [tkschuler/EarthSHAB](https://github.com/tkschuler/EarthSHAB) and [brickworks/mfc-apps](https://github.com/Brickworks/mfc-apps), built on Rust.

## Installation
```sh
cargo install --git https://github.com/Brickworks/yahs.git
```

## Usage
```sh
export RUST_LOG=info # set the logging level [debug, info, warn]
yahs --help
```

## Run a simulated flight
### Configure the sim
Set the simulation configuration in `config/sim_config.toml`

### Start the sim
Then use the `start` command to start a flight simulation. Use the `RUST_LOG`
environment variable to specify the log level to report.
```sh
yahs start
```

### View the flight data
First install the Brickworks support tooling, `firebrick`.
```sh
git clone git@github.com:Brickworks/firebrick.git
cd firebrick
pip install .

firebrick --help
```
Then start up a telemetry dashboard.
```sh
firebrick dashboard -t $PATH_TO_TELEMETRY_CSV
```
Then navigate to the server address specified in the log output.