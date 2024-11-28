# development log

## 2024-11-27

Some of my dependencies may now have Bevy 0.15 support.

- [x] avian3d: (Jondolf/avian:main)[https://github.com/Jondolf/avian/tree/main]
- [ ] bevy_heavy: not supported yet
- [x] iyes_perf_ui:
  (JohnathanFL/iyes_perf_ui:main)[https://github.com/JohnathanFL/iyes_perf_ui/tree/main]
  ([this is open PR from a fork](https://github.com/IyesGames/iyes_perf_ui/pull/22))
- [x] iyes_progress: `0.13.0-rc.1`
- [x] bevy-trait-query:
  [JoJoJet/bevy-trait-query:bevy-0.15-rc](https://github.com/JoJoJet/bevy-trait-query/tree/bevy-0.15-rc)
  (sort of supported, last updated for `bevy-0.15.0-rc.2`)
- [ ] bevy-inspector-egui: not supported yet

Here are some recent projects that I might be able to steal from:

- [bevy-motion-matching](https://github.com/kahboon0425/bevy_motion_matching) -
  angular velocity debug gizmos and egui UI examples for plotting
- [siege](https://github.com/xenon615/siege) - rope example in Avian3d
- [bevy-logging](https://bevy-logging.github.io/) - advanced debug logging &
  tracing example/tutorial

Now that the forces are working and look reasonable, I can start working on the
flight dynamics. I need to start by adding a way to speed up and slow down the
simulation time, because a real flight takes hours. Next I need to add plots so
I can monitor and validate the simulation results for things like altitude,
temperature, and pressure.

- [ ] Physics time multiplier.
- [ ] Plotting gas properties.
- [ ] Plotting balloon kinematics.

Bonus:

- [ ] Add a payload hanging from a tether.
- [ ] Camera follow the balloon, and maybe a scale or some reference in the
      background to illustrate the balloon's size and altitude.

Stretch:

- [ ] Calculate drag forces and moments on arbitrary shapes.
- [ ] Add a wind field.

I figured out how to toggle the physics gizmos at runtime and added force gizmos
that I made myself.

## 2024-11-26

I added a `GasMonitor` to the UI for displaying the gas properties in real time.
With that in place, I can now notice that the balloon's density is reported as
NaN, which is probably why buoyancy is not working. I think I found the bug. The
density was not being updated in the system that updates gas properties from
the atmosphere. Fixed that, but buoyancy is still pegged at 0. Curious, the
buoyant system was querying `With<SimulatedBody>`, maybe something is messed up
with the query because this system is definitely running every frame. A default
`ForceBundle` is supposed to be added to the balloon when it is added to the
world, so I'm curious why the buoyancy component is not being added. The
`Weight` component is being added... Ah, the buoyancy system is also querying
`Volume` components, weight is not. Maybe the query isn't turning up the bodies
because the `Volume` component is not being added? With Bevy 0.15 we can enforce
such a thing with the `#[require(x)]` attribute. Turns out there's no system
that updates the volume because I wanted to calculate it from the balloon's
primitive shape. I'll change buoyancy to query the balloon and get its volume
instead. That fixed the buoyancy system so it runs, but the force results might
not be correct.

```sh
INFO yahs::simulator::forces: Weight [0, -5.1347504, 0]
INFO yahs::simulator::forces: Buoyancy [0, 3888.388, 0]
INFO yahs::simulator::forces: Drag [NaN, NaN, NaN]
```

I traced the drag force NaN to the drag area returning nan from
`balloon.shape.diameter()` at startup before the shape is set up.

```sh
 INFO yahs::simulator::forces::aero: balloon shape: Sphere { radius: NaN }
ERROR yahs::simulator::forces: Drag has NaN magnitude!
ERROR yahs::simulator::forces: Buoyancy has NaN magnitude!
 INFO yahs::simulator::forces::aero: balloon shape: Sphere { radius: 4.2564797 }
 INFO yahs::simulator::forces::aero: balloon shape: Sphere { radius: 4.2810054 }
 INFO yahs::simulator::forces::aero: balloon shape: Sphere { radius: 4.2810054 }
```

I enforced that the mesh volumes are updated before the forces are calculated,
but there might be a few frames at the beginning where the Mesh isn't
initialized yet. Maybe it works a little differently because it's an asset?
I noticed that the balloon was being spawned after the ground plane, and scene
setup was not constrained to run in any system set. So the forces probably ran
before the scene was done loading. I added a few conditions:

- The app starts in the `Loading` state.
- The scene advances to the `Running` state when it is done setting up the scene.
- The physics systems only run when the app is in the `Running` state.

Weird, something else is going on. The balloon shape is a real number when it is
spawned and before the forces run, but when the forces run it is NaN.

```sh
 INFO bevy_winit::system: Creating new window "🎈" (0v1#4294967296)
 INFO yahs::app3d::scene: sphere: Sphere { radius: 0.5 }
 INFO bevy_dev_tools::states: yahs::simulator::core::SimState transition: Some(Loading) => Some(Running)
 INFO yahs::app3d::scene: sim state: Res(State(Running))
 INFO yahs::app3d::scene: balloon spawned: 19v1 Balloon { material_properties: BalloonMaterial { name: "Latex", max_temperature: 373.0, density: 920.0, emissivity: 0.9, absorptivity: 0.9, thermal_conductivity: 0.13, specific_heat: 2000.0, poissons_ratio: 0.5, elasticity: 10000000.0, max_strain: 0.8, max_stress: 500000.0, thickness: 0.0001 }, shape: Sphere { radius: 0.5 } } Sphere { radius: 0.5 }
 INFO yahs::simulator::forces::aero: balloon shape: Sphere { radius: NaN }
ERROR yahs::simulator::forces: Drag has NaN magnitude!
ERROR yahs::simulator::forces: Buoyancy has NaN magnitude!
 WARN avian3d::collision::narrow_phase: 18v1#4294967314 (Ground) and 19v1#4294967315 (Balloon) are overlapping at spawn, which can result in explosive behavior.
 INFO yahs::app3d::scene: sim state: Res(State(Running))
 INFO yahs::simulator::forces::aero: balloon shape: Sphere { radius: 4.2564797 }
 INFO yahs::app3d::scene: sim state: Res(State(Running))
 INFO yahs::simulator::forces::aero: balloon shape: Sphere { radius: 4.2810054 }
 INFO yahs::app3d::scene: sim state: Res(State(Running))
 INFO yahs::simulator::forces::aero: balloon shape: Sphere { radius: 4.2810054 }
```

I noticed that density is initialized to NaN with the ideal gas and the volume
of the balloon is derived from the ideal gas volume. The volume is calculated
before the force, so that explains where this NaN is coming from.

Found it. Ideal gas law has pressure in the denominator of the volume equation.
Pressure initializes to zero by default, so the volume is NaN. Changing the
default from zero to the standard atmospheric pressure fixes that issue.

I think the sim was crashing because it kept defaulting to use way too much
mass in the balloon. I added `with_volume()` to the ideal gas to fix that, so we
can spawn the balloon with a known volume at the default density.

It works!

## 2024-11-24

I found a Bevy 0.15 branch of
[iyes_perf_ui](https://github.com/IyesGames/iyes_perf_ui/pull/22), yay!

I spent some time today practicing with Events and Observers by adding debug UI
toggles. Not very productive but practice is practice.

## 2024-11-23

Now that the basic forces are working, I will add look to adding the other
fundamentals of the flight simulation:

- Ideal gas law, including expansion of gas volume as pressure changes.
- Stats or plots showing the state of the gas and balloon kinematics over time.
- A payload hanging from a tether would be fun too. For this we can lean on the
  Avian [chain_3d](https://github.com/Jondolf/avian/blob/main/examples/chain_3d.rs)
  example.

I upgraded Bevy to 0.15.0-rc.3 and it broke the build, especially with regard to
the avian physics plugins. In migrating to the new Bevy version I simplified
some things, like removing the `crate::properties::Mass` component and instead
using Avian's mass properties. There are some complications because after
upgrading bevy it is crashing due to `Mass` not being a component. I guess mass
properties [being refactored](https://github.com/Jondolf/avian/discussions/499).

The Avian maintainer created a new crate called
[bevy_heavy](https://github.com/Jondolf/bevy_heavy) that contains the new mass
properties tools that allow for directly updating mass properties on primitive
shapes. Then we can use primitive shapes for mass and volume. That simplifies
things by ~~removing mass, volume, and density as separate components~~. Turns
out it is not that simple and having them as separate components is useful and
clean. It is much simpler to write systems that update these components.

[Migrating to Bevy `0.15.0-rc.3`](https://github.com/bevyengine/bevy-website/tree/main/release-content/0.15/migration-guides)
is proving to be a bit of a challenge. The main feature from it that I want to
use is the new separation of meshes from rendering, since then we can use meshes
for calculations like volume and drag even if we want to use a CLI and don't
want to render anything. This feature also led to some welcome improvements to
Bevy's meshing tools.

Something about 0.15 seems to have broken bundles. I wonder if the API changed
or if there's some old garbage hanging around that is causing issues. I wiped
the cache and rebuilt the project (`cargo clean && cargo build`). It turns out
the reason that builds are failing is because some of the 3rd party dependencies
I'm using are not compatible with the new Bevy version. I need to go through and
update them or remove them.

- [x] `avian3d` -> branch `bevy-0.15`
- [ ] ~~`bevy_heavy`~~ remove for now
- [x] `bevy-trait-query` -> branch `bevy-0.15-rc`
- [ ] ~~`bevy_common_assets`~~ remove for now
- [ ] ~~`bevy_panorbit_camera`~~ remove for now
- [ ] ~~`iyes_perf_ui`~~ remove for now
- [ ] ~~`bevy-inspector-egui`~~ remove for now

I also removed `serde` from the dependencies. I won't be getting to config files
any time soon.

It's probably better to not use so many 3rd party plugins. Fortunately most of
these are debug tools and not essential to the simulator.

This demo does a great job using Egui and debug vectors:
[bevy_motion_matching](https://github.com/kahboon0425/bevy_motion_matching)
something to look into later.

I'm gonna do it. I'm going to make the simulator and the 3D app separate crates.
There are three reasons for this:

1. The simulator crate can be used as a library in other projects.
2. The 3D app can be built and run independently of the rest of the code.
3. I want faster compile times.

Nevermind, it complicated things way too much and was distracting. In the future
it might be worthwhile but it's probably best to just use feature flags anyway.

## 2024-11-18 again

I think I was a bit naive to install `bevy-trait-query`. It works for now but in
the future we should really move away from it. It is currently a crutch.

For some reason when the balloon bounces, it accelerates upward to oblivion. I
added some errors to stop the simulation when the balloon goes out of bounds,
but right now it panics. Not great but better than nothing. Best I can do is
bring in the "out of bounds" level to be less than the true bounds.

The drag force is the one that is causing the bad acceleration. Weight and
buoyancy don't change with time since the balloon has constant size right now.
Annoying since the drag is supposed to be the placeholder.

I found the bug. The drag's "flow velocity" variable was being set to the
_balloon_ velocity, rather than the opposing force. After making that change,
the balloon gently floats to the ground when filled with air, and gently rises
when filled with helium.

- Added pause/play controls. Default key is `Space`.
- Added a new ui that displays the simulation state.
- Added a new ui that displays forces in real time, assuming only one balloon.
- Added an `Anomaly` state to the sim that is supposed to freeze.

## 2024-11-18

Iterating with AI on the drag calculations. It is set up to perform a raycast
along the velocity vector and compute the drag as a function of the angle of
attack. Each raycast hits a point on the surface of the collider and the normal
and differential area is used to compute the drag force for each one.

Turns out air viscosity also changes with altitude for the standard atmosphere.
I learned today that there is also _dynamic_ viscosity and _kinematic_
viscosity. The latter is the former divided by the density. So as density
changes, so too does viscosity. These two types are really just different ways
to express the same thing.

In any case, I will wire up drag to be calculated from the bounding-sphere of
the collider. That way I can get a simple drag calculation working before
wiring up the more complex stuff and ignore things like shear and asymmetry for
now.

I reorganized the module hierarchy again shut up I'm not the problem you are.

- Split the `forces` module into `body` and `aero`. The base `forces` module
  contains the common code for all forces.
- Added `AeroPlugin` for computing drag on solid bodies.
- Added `BodyForcesPlugin` that deals with forces related to the mass and
  volume of the rigid bodies.
- Added `ForceVisualizationPlugin` for visualizing the forces as vectors in the
  scene.
- Moved serde features behind the `config-files` feature flag. I'm hoping to
  pare down the default dependencies for the project.
- Added `intro_to_drag.md` as I learn how to compute drag on arbitrary shapes.
- Added `camera.rs` as a simple pan-orbit camera controller.
- Added `controls.rs` for managing keybindings, and `KeyBindingsConfig` resource
  for storing the keybindings. I use a resource instead of constants because
  it makes the keybindings easier to change at runtime and feels more natural
  alongside how Bevy handles inputs anyway.
- Moved `scene` and `ui` modules to a `graphics` module. Hopefully this will
  make it easier to separate the concerns of physics simulation and graphics,
  like if I ever want to add a TUI or CLI instead of a 3D graphics UI.

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

Update: it was not better.

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
