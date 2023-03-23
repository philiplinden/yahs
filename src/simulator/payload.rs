// ----------------------------------------------------------------------------
// Payload
// -------
// Properties and functions of the balloon's payload and avionics
// ----------------------------------------------------------------------------

pub struct Bus {
    pub dry_mass: f32, // kg
    pub drag_coeff: f32, // drag coefficient during free fall
    pub drag_area: f32, // effective area used for drag, m^2
}
