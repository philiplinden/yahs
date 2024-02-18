// ----------------------------------------------------------------------------
// Bus
// ---
// Properties and functions of the balloon's structure and avionics busses
// ----------------------------------------------------------------------------

use super::{
    config::{BodyConfig, ParachuteConfig},
    forces,
    gas::Atmosphere,
    SolidBody,
};


#[derive(Copy, Clone)]
pub struct Body {
    mass: f32,       // kg
    drag_coeff: f32, // drag coefficient
    drag_area: f32,  // effective area used for drag, m^2
}

impl Body {
    pub fn new(config: BodyConfig) -> Self {
        Self {
            mass: config.mass_kg,
            drag_coeff: config.drag_coeff,
            drag_area: config.drag_area_m2,
        }
    }
}

impl SolidBody for Body {
    fn drag_area(&self) -> f32 {
        self.drag_area
    }
    fn drag_coeff(&self) -> f32 {
        self.drag_coeff
    }
    fn total_mass(&self) -> f32 {
        self.mass
    }
}


#[derive(Copy, Clone)]
enum ParachuteState {
    Stowed,
    PartiallyOpen,
    FullyOpen,
}


#[derive(Copy, Clone)]
pub struct Parachute {
    state: ParachuteState,
    mass: f32,         //kg
    drag_coeff: f32,   // drag coefficient
    deploy_force: f32, // force needed to go from partial to full open, N
    drag_area: f32,        // effective area used for drag, m^2
    deploy_progress: f32,  // percentage of how open the parachute is
    deploy_rate: f32,      // progress increment per time-step
}

impl SolidBody for Parachute {
    fn drag_area(&self) -> f32 {
        match self.state {
            ParachuteState::Stowed => 0.0,
            ParachuteState::PartiallyOpen => self.drag_area * self.deploy_progress,
            ParachuteState::FullyOpen => self.drag_area,
        }
    }
    fn drag_coeff(&self) -> f32 {
        self.drag_coeff
    }
    fn total_mass(&self) -> f32 {
        self.mass
    }
}

impl Parachute {
    fn deploy(&mut self, force: f32) {
        match self.state {
            ParachuteState::Stowed => {
                if force >= self.deploy_force {
                    self.state = ParachuteState::PartiallyOpen
                }
            }
            ParachuteState::PartiallyOpen => self.continue_deploying(),
            ParachuteState::FullyOpen => {}
        }
    }

    fn continue_deploying(&mut self) {
        self.deploy_progress += &self.deploy_rate;
        if self.deploy_progress >= 1.0 {
            self.state = ParachuteState::FullyOpen
        }
    }
}


#[derive(Copy, Clone)]
pub struct ParachuteSystem {
    pub main: Parachute,
    pub drogue: Parachute,
}

impl ParachuteSystem {
    pub fn new(config: ParachuteConfig, timestep_size: f32) -> Self {
        let main = Parachute {
            mass: config.total_mass_kg,
            drag_coeff: config.main_drag_coeff,
            drag_area: config.main_area_m2,
            deploy_force: config.deploy_force_n,
            deploy_progress: 0.0,
            deploy_rate: config.deploy_time_s / timestep_size,
            state: ParachuteState::Stowed,
        };
        let drogue = Parachute {
            mass: 0.0,
            drag_coeff: config.drogue_drag_coeff,
            drag_area: config.drogue_area_m2,
            deploy_force: 0.0,
            deploy_progress: 0.0,
            deploy_rate: 1.0,
            state: ParachuteState::Stowed,
        };
        Self { main, drogue }
    }

    pub fn total_mass(&self) -> f32 {
        self.main.mass + self.drogue.mass
    }

    pub fn deploy(&mut self, atmo: Atmosphere, velocity: f32) {
        self.drogue.deploy(100.0);
        let drogue_drag = forces::drag(atmo, velocity, self.drogue);
        self.main.deploy(drogue_drag);
    }
}
