# yet another HAB simulator

A high altitude balloon flight simulator based on [tkschuler/EarthSHAB](https://github.com/tkschuler/EarthSHAB) and [brickworks/mfc-apps](https://github.com/Brickworks/mfc-apps), built on Rust.

## Installation
```sh
cargo install --git https://github.com/Brickworks/yahs.git --features gui
yahs --help
```

## Usage (CLI)
Set the simulation configuration in `config/default.toml`

Then use the `start` command to start a flight simulation. Use the `RUST_LOG`
environment variable to specify the log level to report.

```sh
yahs start
```

## Usage (GUI)

```sh
yahs gui
```
