# buoy

A buoyancy simulation environment built with [Bevy](https://bevyengine.org/) in
Rust.

## Usage

### As a library

Add `buoy` to your `Cargo.toml`:

```toml
[dependencies]
buoy = "0.1.0"
```

Then, add the `BuoyPlugin` to your Bevy app:

```rust
use buoy::prelude::BuoyPlugin;

fn main() {
    App::new().add_plugins(BuoyPlugin);
}
```

Then you can use all of the components and systems from the
[`buoy-core`](./crates/buoy-core/README.md) crate.


### As an application

Running this package as a standalone application compiles all of the crates
and runs the default interface:

```bash
cargo run
```

You can force the standalone application to run a particular interface too:

```bash
cargo run --bin buoy-ui
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
