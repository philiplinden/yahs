# development log

## 2024-11-17

Time to see what's going on with the forces through some new UIs instead of
relying on the inspector plugin. It would be nice to have some gizmos that show
the forces as vectors in the scene.

I added a new plugin that lets me query components by a trait. This greatly
simplifies the logic for collecting forces, and makes the coupling between the
force components and the equations that define them more direct.

I used the [Newtonian Aerodynamics for General Body Shapes](https://ntrs.nasa.gov/citations/19780014285)
paper to help me understand the equations for computing drag. o1-mini helped
interpret the equations and implement them. A summary of the paper is saved in
[drag.md](drag.md). I overcomplicated the drag calculation by sampling points on
the surface of the collider and then projecting them onto the surface but it was
fun to implement.

## 2024-11-16

I figured out how to get the forces to update correctly setting `WeightForce`,
`BuoyantForce`, and `DragForce` to wrap an `ExternalForce` component was a mistake.
Instead, I had to change them to wrap a `Vec3`, which is the input to the
`set_force` method. What I'm doing now is having systems that update the forces,
then after that run the system that adds the forces into a single vector. That
is what gets applied to the rigid body.

Remember that `WeightForce`, `BuoyantForce`, and `DragForce` are _components_,
not forces themselves. We name them so we can query the _kinds_ of forces,
otherwise generic force vectors would not be distinguishable from each other.
The alternative (when we can't tell which vector is which) is to add the vectors
to a "blind" array and then update the `ExternalForce` component as before,
except that when we sum them we don't know which vector is which. Now that I
think about it, that might be easier and simpler in the long run. If we need to
know each force's value and identity, we can add it to a resource or something
that the UI can query.

Update: it was not better. Still ran into the forever accumulating force issue
using a `ForceCollection` aggregator component and generic vectors. However, I
found a way to 
I fell in love with [iyes_perf_ui](https://github.com/IyesGames/iyes_perf_ui).
It is simple and easy to work with. Egui was adding a lot of overhead, more than
I want right now. This is perfect until I'm done with the basic features of the
simulation.

- Fixed systems for computing the weight, buoyancy, and drag forces.
- Dropped egui.
- Added `iyes_perf_ui` for performance monitoring and other UI.

## 2024-11-14

My focus today is to spend some time getting the balloon to "look right". Things
like the balloon's shape, skin material, and thickness. Also things like the
tether and a payload box. My plan for balloon material stress is to calculate
stress from the change in surface area. The gas volume can change shape all it
wants, as long as the density follows the ideal gas law. The balloon skin will
stretch and deform to follow the gas volume shape, and the stress will be
calculated from the change in surface area of the skin's mesh.

My goal at the end of today is to see a balloon with a little payload hanging
from it that slowly rises into the air.

In Avian, forces persist across frames by default. The way the forces are set up
now ends up applying the same force every frame, which is accumulating and not
correct. I think there are also some issues with applying forces in separate
systems. I'm not entirely sure what the best way to handle this is. I think if
we remove the persistence assumption then every frame the only forces applied
are the ones that _we_ explicitly apply in systems. This situation is more
complex than the simple case Avian is pre-configured for, like 1G of gravity
down and maybe some friction.

I'm second-guessing whether I should use a new `ExternalForce` component for
each force instead of mutating the default component that spawns in when the
rigid body is created. The default component is persistent by default, and
this is probably why the forces are accumulating. It feels gross to have to
clear the force every frame or apply an impulse every frame instead of just
updating the force.

- Added a bunch of handy geometry functions to `properties.rs`.
- Moved the `PbrBundle` into the `BalloonBundle` so the balloon is more tightly
  associated with its mesh, material, and transform. Here the PBR (physics based
  rendering) is not relevant to the physics simulation, it just happens to be
  the bundle that contains these components.
- Added `PayloadPlugin` for spawning a payload and tether. Placeholder for now.
- Removed `#![allow(dead_code)]` from a few modules as I remove placeholders.

## 2024-11-13

Avian has its own `Mass` component but it is not as convenient for this project
as something custom that can be updated with the other thermodynamics properties
like temperature and pressure. The Avian component is not reflected in the
inspector plugin so it's hard to debug, too. I can make a custom `Mass` that
hooks up to the Avian one and then otherwise is whatever I want.

I realized I was using `ExternalForce` incorrectly. Instead of adding more
force components, I should be applying additional vectors to the existing
`ExternalForce` component using the `apply_force` method.

- Added names to the all entities in the test scene.
- Moved core properties (Temperature, Pressure, Volume, Density, Mass) to their
  own module called `properties.rs` and added a plugin for registering them. To
  make things more ergonomic, these properties are exposed at the top level of
  the simulator plugin so that they can be used in other systems.
- Removed the `thermodynamics` module since it became redundant.
- Added a `CorePhysicsPlugin` for registering the physics plugins.
- Added a `ScenePlugin` for setting up the scene with the HAB and atmosphere.
- Added systems to update the density, weight, and buoyant forces every frame.
  This wasn't working until I added the default Avian physics plugins,
  obviously.
- Got buoyancy and weight forces working on the collider. When the gas species
  is set to air, the balloon slowly sinks to the ground. When the gas species
  is set to helium, the balloon floats.

## 2024-11-11

More tightening up of the systems using `bevy-inspector-egui` and making sure
the numbers update correctly.

I am considering reducing the `IdealGas` component to be paired with a rigid
body. The rigid body would have a `Mass` and a `Collider` (sphere, ellipsoid,
etc.) and the `IdealGas` would have a `Volume` and a `GasSpecies`. This would
allow for more flexibility in how the gas volume is computed and updated. For
example, if the gas volume is not attached to the rigid body it may be
desirable to compute the volume based on the mesh instead. It would also allow
for more complex gas interactions with the rigid body, like inside a parachute.

Converted forces to use the new `ExternalForce` component from Avian.

- Renamed `mechanics` module to `forces`
- Corrected the buoyancy force calculation to use only the ambient density of
  the atmosphere and not the density of the gas inside the balloon. The weight
  of the gas is already accounted for in the `mass` of the gas.
- Added the `Component` trait to `Temperature`, `Pressure`, `Density`, and
  `Volume`.
- Removed `config.rs` since its duties are fulfilled by `assets.rs`

## 2024-11-10

A little bit of cleanup to get the thermodynamics plugin functional and neat.
Also getting the balloon to appear as a simple sphere in the simulation but it's
annoyingly full of boilerplate and other setup unrelated to the physics.

It's a lot harder than I thought to compute the volume of a mesh, mostly because
meshes are surfaces and not volumes.
[Possible, but not simple.](https://github.com/dimforge/parry/blob/68abfc1c22c0beb6d8eba11d57acbb29b4837577/src/mass_properties/mass_properties_trimesh3d.rs#L155)
Instead of having everything be _directly_ coupled, it is suffiecient to have
all of these systems (volume, mesh, collider, etc.) update together based on
the same state.

Instead, I'll make the debug view show the current atmospheric conditions at a
given point in the simulation.

- Added an About window.
- Added `bevy-inspector-egui` plugin for debugging.
- Removed `VolumetricBody` from `IdealGas` to make things more direct.
- Removed `compute_volume_from_mesh` function.

## 2024-11-09

Pushing forward toward bringing the basic features into Bevy ECS. Today I am
focusing on the ideal gas law and dynamics functions.

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

- Added splash screen to the application.
- Changed the generic asset loader to a configuration loader for now.
- Added asset tracking plugin.
- Added dev tools provided by the Bevy Quickstart Template.
- Added `dev` feature flag and Bevy build optimiztions to `Cargo.toml`.
- Added `lib.rs` and moved some things around to clean up the root directory.
- Replaced all logging with bevy's built-in logging plugin.

## 2024-11-07

I am switching to Bevy for the simulation. Bevy is a game engine built upon an
ECS framework. It allows for high performance, multi-threaded, dynamic
simulations with 3D graphics and rich interactive GUIs.

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
