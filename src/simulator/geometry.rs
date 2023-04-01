
// ----------------------------------------------------------------------------
// Geometry
// --------
// A collection of geometric calculations.
// ----------------------------------------------------------------------------
use std::f32::consts::PI;

pub fn sphere_volume(radius: f32) -> f32 {
    (4.0 / 3.0) * PI * libm::powf(radius, 3.0)
}

pub fn shell_volume(internal_radius: f32, thickness: f32) -> f32 {
    let external_radius = internal_radius + thickness;
    let internal_volume = sphere_volume(internal_radius);
    let external_volume = sphere_volume(external_radius);
    external_volume - internal_volume
}

pub fn sphere_radius_from_volume(volume: f32) -> f32 {
    libm::powf(volume, 1.0 / 3.0) / (4.0 / 3.0) * PI
}

pub fn sphere_surface_area(radius: f32) -> f32 {
    4.0 * PI * libm::powf(radius, 2.0)
}

pub fn projected_spherical_area(volume: f32) -> f32 {
    // Get the projected area (m^2) of a sphere with a given volume (m^3)
    libm::powf(libm::powf(volume / (PI * (4.0 / 3.0)), 1.0 / 3.0), 2.0) * PI
}