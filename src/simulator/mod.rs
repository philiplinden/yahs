pub mod balloon;
pub mod bus;
pub mod config;
pub mod constants;
pub mod forces;
pub mod gas;
pub mod heat;
pub mod io;
pub mod schedule;

pub trait SolidBody {
    fn drag_area(&self) -> f32;
    fn drag_coeff(&self) -> f32;
    fn total_mass(&self) -> f32;
}
