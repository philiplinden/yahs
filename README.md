# yet another HAB simulator

A high altitude balloon flight simulator built in
[Bevy](https://bevyengine.org/) with Rust, inspired by
[tkschuler/EarthSHAB](https://github.com/tkschuler/EarthSHAB).

[devlog](docs/devlog.md)

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
[`yahs-simulator`](./src/simulator/README.md) crate.


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

## License

Except where noted (below and/or in individual files), all code in this
repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or
  [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option. This means you can select the license you prefer! This
dual-licensing approach is the de-facto standard in the Rust ecosystem and there
are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to
include both.
