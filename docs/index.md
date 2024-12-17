# yet another HAB simulator

This is a high altitude balloon simulator built in
[Bevy](https://bevyengine.org/). The goal is to simulate the flight of a HAB
(High Altitude Balloon) and to provide a software-in-the-loop platform for
testing out various physics and engineering concepts with a realistic
simulation.

This project is split into three parts:

- [yahs](crates/yahs/README.md) is the main simulation crate. It is a Bevy
  plugin that can be added to any Bevy project.
- [yahs-ui](crates/yahs-ui/README.md) is a simple UI for the simulation. It is
  not required but it is useful for visualizing the simulation and for
  simulation and for debugging.
- [yahs-cli](crates/yahs-cli/README.md) is a command line tool for running the
  simulation. It is useful for testing out the simulation without having to deal
  with the UI or for running the simulation in a headless mode.


## Usage

### As a library

Add `yahs` to your `Cargo.toml`:

```toml
[dependencies]
yahs = "0.4.0"
```

Then, add the `SimulatorPlugins` to your Bevy app:

```rust
use yahs::prelude::SimulatorPlugins;

fn main() {
    App::new().add_plugins(SimulatorPlugins);
}
```

Then you can use all of the components and systems from the
[`yahs`](./crates/yahs/README.md) crate.


### As an application

Running this package as a standalone application compiles all of the crates
and runs the CLI by default:

```bash
cargo run
```

Force the standalone application to run the GUI instead of the CLI:

```bash
cargo run --bin yahs-ui
```
