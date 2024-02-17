// ----------------------------------------------------------------------------
// Bus
// ---
// Properties and functions of the balloon's structure and avionics busses
// ----------------------------------------------------------------------------

#![allow(dead_code)]

pub struct Bus {
    pub dry_mass: f32, // kg
    pub drag_coeff: f32, // drag coefficient during free fall
    pub drag_area: f32, // effective area used for drag, m^2
}
