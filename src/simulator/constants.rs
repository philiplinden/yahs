// ----------------------------------------------------------------------------
// Cosntants
// ---------
// Physical constants (read-only).
// ----------------------------------------------------------------------------
pub const STANDARD_TEMPERATURE: f32 = 273.15; // [K]
pub const STANDARD_PRESSURE: f32 = 101325.0; // [Pa]
pub const BOLTZMANN_CONSTANT: f32 = 1.38e-23_f32; // [J/K]
pub const AVOGADRO_CONSTANT: f32 = 6.022e+23_f32; // [1/mol]
pub const R: f32 = BOLTZMANN_CONSTANT * AVOGADRO_CONSTANT; // [J/K-mol] Ideal gas constant

pub const STANDARD_G: f32 = 9.80665; // [m/s^2] standard gravitational acceleration
pub const EARTH_RADIUS_M: f32 = 6371007.2; // [m] mean radius of Earth

pub const ATMOSPHERE_MOLAR_MASS: f32 = 0.02897; // [kg/mol] molar mass of air
