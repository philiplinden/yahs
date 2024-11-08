# development log

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
