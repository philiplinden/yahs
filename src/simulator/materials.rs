// ----------------------------------------------------------------------------
// Materials
// ---------
// Properties, attributes and functions related to the material properties.
// Source: https://www.matweb.com/
// ----------------------------------------------------------------------------

#[derive(Clone, PartialEq)]
pub struct Material {
    pub max_temperature: f32, // temperature (K) where the given material fails
    pub density: f32, // density (kg/m^3)
    pub emissivity: f32, // emissivity coefficient of the material for blackbody light
    pub thermal_conductivity: f32, // thermal conductivity (W/mK) of the material at room temperature
    pub max_elongation: f32, // elongation at failure (decimal, unitless) 1 = original size
    pub max_stress: f32, // tangential stress at failure (Pa)
}

pub const NOTHING: Material = Material {
    // nothing
    max_temperature: f32::INFINITY,
    density: 0.0,
    emissivity: 1.0,
    thermal_conductivity: f32::INFINITY,
    max_elongation: f32::INFINITY,
    max_stress: f32::INFINITY,
};

pub const RUBBER: Material = Material {
    // Natural Rubber, Vulcanized (NR, IR, Polyisoprene)
    max_temperature: 400.0,
    density: 950.0,
    emissivity: 0.86,
    thermal_conductivity: 0.34,
    max_elongation: 8.0,
    max_stress: 150_000_000.0,
};

pub const LOW_DENSITY_POLYETHYLENE: Material = Material {
    // Low Density Polyethylene (LDPE), Film Grade
    max_temperature: 380.0,
    density: 910.0,
    emissivity: 0.94,
    thermal_conductivity: 0.15,
    max_elongation: 1.0,
    max_stress: 300_000_000.0,
};
