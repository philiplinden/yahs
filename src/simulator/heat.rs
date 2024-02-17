// ----------------------------------------------------------------------------
// Heat
// ----
// Heat transferred between and stored in materials
// 
//  Assumptions:
// 1. convection heat transfer is linear
//    This is reasonable for airflows with velocity under 5 m/s
// 2. boundary nodes do not change temperature
// 3. thermal resistances are constant between nodes
//
// https://www.sciencedirect.com/science/article/pii/S1000936118301018
// https://materion.com/-/media/files/alloy/newsletters/technical-tidbits/issue-no-114-thermal-emissivity-and-radiative-heat-transfer.pdf
// ----------------------------------------------------------------------------

#![allow(dead_code)]
#![allow(unused_imports)]

use super::balloon::Balloon;
use super::gas::Atmosphere;

fn absorbed(balloon: Balloon, heat_flux: f32) -> f32 {
    // absorbed
    // A * q''
    let surface_area: f32 = balloon.surface_area();
    surface_area * heat_flux
}

// convected
// A * h * (T - Tatmo)

// emitted
// eps * sigma * A * (T^4 - Tatmo^4)

// stored
// rho * V * cp * dT/dt