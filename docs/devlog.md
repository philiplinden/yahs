# development log

## 2024-11-09

Pushing forward toward bringing the basic features into Bevy ECS. Today I am
focusing on the ideal gas law and dynamics functions.

### Changelog - 2024-11-09

- Moved basic equations around, like into a new thermodynamics module
- Added custom structs for `Temperature`, `Pressure`, `Volume`, `Density`, and
  `MolarMass` to abstract unit conversions and such. It is safer to work with
  structs than bare `f32`s for physics equations.

## 2024-11-08

Focusing on getting the app running as a simple splash screen followed by a
loading screen followed by the running application. Loading the configuration
file and setting up the simulation from there.

I am having trouble getting the Bevy asset server to load the configuration
file. It seems to be advancing to the running state before the configuration
file is loaded. I added a splash screen to practice with state transitions.

I also added dev tools provided by the Bevy Quickstart Template
([link](https://github.com/TheBevyFlock/bevy_new_2d/blob/main/src/dev_tools.rs))
to help with debugging. This is toggled with the `F3` key by default, and only
added when the `dev` feature is enabled (it is not enabled by default when
building with cargo and omitted from the release build). I borrowed some other
patterns from the Bevy Quickstart Template for the asset tracking and a few
small things.

I'm having trouble integrating simple things like loading in values from the
config files. I can't tell what's getting in my way. Time to move things out of
configs and into hard-coded statics.

Another note about migrating to Bevy is that the engine assumes we are doing
graphics outside of a terminal. We could implement a terminal UI or continue to
support a CLI, but that's 2xs or 3x the work to maintain multiples. Since I plan
to leverage the 3D physics (with things like raycasting) we will drop CLI
support for now.

### Changelog - 2024-11-08

- Added splash screen to the application.
- Changed the generic asset loader to a configuration loader for now.
- Added asset tracking plugin.
- Added dev tools provided by the Bevy Quickstart Template.
- Added `dev` feature flag and Bevy build optimiztions to `Cargo.toml`.
- Added `lib.rs` and moved some things around to clean up the root directory.
- Replaced all logging with bevy's built-in logging plugin.

## 2024-11-07

I am switching to Bevy for the simulation. Bevy is a "bevy engine" which is A
framework for building games and simulations. It allows for high performance,
multi-threaded, dynamic simulations.

The first reason is that I want to be able to spend more time on the
interactions between the HAB components and less time on the fundamental physics
and simulation scheduling loop. Bevy has a very nice schedule system that allows
for easy parallelization of systems. It also has a component system that allows
me to keep all the logic for the physics systems close to the objects that they
act on. For example, all of the solid bodies that will need to have drag applied
will have a `Body` component, and the logic to calculate the drag on those
bodies will be computed from the their mesh using custom colliders and forces on
top of the physics engine, [Avian](https://github.com/Jondolf/avian), that takes
care of equations of motion, collisions, and constraints.

The second reason is that I want to be able to run the simulation in a web
browser. Bevy has a web backend that allows for this and very nice tools for
visualizing the simulation state. It also has first-class support for
[Egui](https://github.com/emilk/egui) which is a library for building
interactive GUIs with [Bevy Egui](https://github.com/mvlabat/bevy_egui), and
first-class support for loading assets like configs, 3D models, and textures.

The first thing I want to do is to get a simple ballistic flight working in
Bevy so that I can validate the fundamental physics assumptions that I have
made. To do this I'll need to start duplicating some of the functionality that
I had in the previous Rust simulation. Namely, I need to:

1. Initialize the atmosphere.
2. Create the solid bodies (balloon, parachute, payload).
3. Set up a schedule for computing the physics updates every tick/frame.

The Bevy schedule system will completely replace the threaded event loop that I
had in the previous simulation, including things like `SimCommands`,
`SimOutput`, `SimInstant`, and the `AsyncSim` struct.

### Changelog - 2024-11-07

- **Main Application (`src/main.rs`):**
  - Integrated `simulator::SimulatorPlugins` to replace the previous UI plugins
    setup.

- **Atmosphere Module (`src/simulator/atmosphere.rs`):**
  - Introduced a new `Atmosphere` struct to model atmospheric conditions using
    the US Standard Atmosphere, 1976.
  - Added functions for calculating temperature and pressure based on altitude.

- **Balloon Module (`src/simulator/balloon.rs`):**
  - Added `BalloonPlugin` struct implementing the `Plugin` trait for Bevy
    integration.

- **Forces Module (`src/simulator/forces.rs`):**
  - Updated to use the new `atmosphere::Atmosphere` for buoyancy and drag
    calculations.

- **Gas Module (`src/simulator/gas.rs`):**
  - Moved `Atmosphere` struct to its own module.
  - Added utility functions for calculating gas volume and density.

- **Simulator Module (`src/simulator/mod.rs`):**
  - Added new modules: `atmosphere` and `units`.
  - Converted `SimulatorPlugin` to `SimulatorPlugins` as a `PluginGroup`.

- **Units Module (`src/simulator/units.rs`):**
  - Added a utility function to convert Celsius to Kelvin.
