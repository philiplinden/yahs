# yet another HAB simulator

This is a high altitude balloon simulator built in
[Bevy](https://bevyengine.org/). The goal is to simulate the flight of a HAB
(High Altitude Balloon) and to provide a software-in-the-loop platform for
testing out various physics and engineering concepts with a realistic
simulation.

This project is split into three parts:

- [yahs-sim](https://github.com/philiplinden/yahs-sim) is the main simulation
  crate. It is a Bevy plugin that can be added to any Bevy project.
- [yahs-ui](https://github.com/philiplinden/yahs-ui) is a simple UI for the
  simulation. It is not required but it is useful for visualizing the
  simulation and for debugging.
- [yahs-cli](https://github.com/philiplinden/yahs-cli) is a command line tool
  for running the simulation. It is useful for testing out the simulation
  without having to deal with the UI.
