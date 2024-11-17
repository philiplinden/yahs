use avian3d::{math::{FRAC_PI_2, TAU}, prelude::*};
use bevy::prelude::*;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use bevy_trait_query::{self, RegisterExt};

use super::{Atmosphere, Density, Mass, Volume, ForceUpdateOrder, Force};

pub struct AeroPlugin;

impl Plugin for AeroPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Drag>();
        app.register_component_as::<dyn Force, Drag>();
        app.init_resource::<SurfaceSamplingConfig>();
        app.add_systems(Update, update_drag_parameters.in_set(ForceUpdateOrder::Prepare));
    }
}

/// Force (N) due to drag as a solid body moves through a fluid.
#[derive(Component, Reflect)]
pub struct Drag {
    local_gravity: f32,
    drag_normal: Vec3,
    ambient_density: Density,
    drag_area: f32,
    drag_coeff: f32,
}
impl Default for Drag {
    fn default() -> Self {
        Self {
            local_gravity: 0.0,
            drag_normal: Vec3::ZERO,
            ambient_density: Density::ZERO,
            drag_area: 0.0,
            drag_coeff: 0.0,
        }
    }
}
impl Drag {
    pub fn update(
        &mut self,
        ambient_density: Density,
        drag_normal: Vec3,
        drag_area: f32,
        drag_coeff: f32,
    ) {
        self.ambient_density = ambient_density;
        self.drag_normal = drag_normal;
        self.drag_area = drag_area;
        self.drag_coeff = drag_coeff;
    }
}
impl Force for Drag {
    fn force(&self) -> Vec3 {
        drag(
            self.ambient_density.kg_per_m3(),
            self.drag_normal,
            self.drag_area,
            self.drag_coeff,
        )
    }
    fn point_of_application(&self) -> Vec3 {
        Vec3::ZERO
    }
}

fn update_drag_parameters(
    atmosphere: Res<Atmosphere>,
    mut bodies: Query<(&mut Drag, &Position, &LinearVelocity, &Collider), With<RigidBody>>,
    sampling_config: Res<SurfaceSamplingConfig>,
) {
    let num_samples = sampling_config.num_samples;

    let wind_velocity = Vec3::ZERO; // TODO: update with actual wind

    for (mut drag, position, velocity, collider) in bodies.iter_mut() {
        let ambient_density = atmosphere.density(position.0);
        
        // Calculate relative flow velocity
        let relative_flow_velocity = velocity.0 - wind_velocity;

        // Update drag component with normalized force direction
        let drag_force = aero_drag_from_collider(collider, relative_flow_velocity, ambient_density, num_samples);
        let drag_normal = if drag_force == Vec3::ZERO { Vec3::ZERO } else { -drag_force.normalize() };

        drag.update(
            ambient_density,
            drag_normal,
            1.0, // Area handled in aero_drag_from_collider
            1.0, // Coefficient handled in aero_drag_from_collider
        );
    }
}

/// Force (N) due to drag as a solid body moves through a fluid.
pub fn drag(ambient_density: f32, drag_normal: Vec3, drag_area: f32, drag_coeff: f32) -> Vec3 {
    drag_normal * drag_coeff / 2.0
        * ambient_density
        * drag_normal.length_squared()
        * drag_area
}

/// Number of samples for surface sampling.
#[derive(Resource, Reflect)]
pub struct SurfaceSamplingConfig {
    num_samples: usize,
}
impl Default for SurfaceSamplingConfig {
    fn default() -> Self {
        SurfaceSamplingConfig { num_samples: 100 }
    }
}

/// Represents a sampled point on the collider's surface along with its normal and differential area.
struct SurfaceSample {
    point: Vec3,
    normal: Vec3,
    differential_area: f32,
}

/// Calculates the aerodynamic drag force vector for a given collider, velocity, and wind velocity.
///
/// # Parameters
///
/// - `collider`: Reference to the collider object.
/// - `velocity`: The velocity of the object relative to the wind.
/// - `wind_velocity`: The velocity of the wind.
///
/// # Returns
///
/// A `Vec3` representing the drag force vector.
pub fn aero_drag_from_collider(
    collider: &Collider,
    relative_flow_velocity: Vec3,
    ambient_density: Density,
    num_samples: usize,
) -> Vec3 {
    // Handle case where flow_velocity is near zero to avoid invalid rotations
    let view_dir = relative_flow_velocity.normalize_or_zero();
    if view_dir.length_squared() < 1e-6 {
        return Vec3::ZERO;
    }

    // Sample surface points along with their differential areas using simplified iterator
    let surface_samples = sample_surface_with_area(collider, num_samples, view_dir);

    // Calculate net drag using differential areas with optimized parallel processing
    calculate_net_drag(&surface_samples, relative_flow_velocity, ambient_density)
}

/// Samples points on the collider's surface and calculates their differential areas.
///
/// # Parameters
///
/// - `collider`: Reference to the collider object.
/// - `samples`: Number of samples to generate.
/// - `view_dir`: The normalized direction vector for sampling.
///
/// # Returns
///
/// A vector of `SurfaceSample` structs.
fn sample_surface_with_area(
    collider: &Collider,
    samples: usize,
    view_dir: Vec3,
) -> Vec<SurfaceSample> {
    // Total solid angle for a hemisphere
    let total_solid_angle = std::f32::consts::PI * 2.0;
    // Solid angle per sample
    let delta_omega = total_solid_angle / samples as f32;

    // Precompute the rotation quaternion to align with view_dir
    let rotation = Quat::from_rotation_arc(Vec3::Z, view_dir);

    // Use a parallel iterator for performance
    (0..samples)
        .into_par_iter()
        .filter_map(|_| {
            // Initialize a separate RNG within each thread to ensure thread safety
            let mut rng = thread_rng();
            let theta = rng.gen::<f32>() * FRAC_PI_2; // Hemisphere: 0 to π/2
            let phi = rng.gen::<f32>() * TAU;

            // Spherical coordinates to Cartesian coordinates
            let dir = Vec3::new(theta.sin() * phi.cos(), theta.sin() * phi.sin(), theta.cos());

            // Rotate the direction to align with the view direction
            let sampled_dir = rotation * dir;

            // Project the sampled direction onto the collider's surface
            let far_point = sampled_dir * 1000.0; // Far point in the sampled direction
            let (projected_point, _) = collider.project_point(
                Vec3::ZERO,        // translation
                Quat::IDENTITY,    // rotation
                far_point,
                false,             // treat as hollow
            );

            // Compute the normal at the sampled point
            let normal = projected_point.normalize_or_zero();

            // Calculate the angle theta between view_dir and normal
            let cos_theta = normal.dot(view_dir).max(0.0);
            if cos_theta > 0.0 {
                // Differential area calculation
                let differential_area = delta_omega / cos_theta;
                Some(SurfaceSample {
                    point: projected_point,
                    normal,
                    differential_area,
                })
            } else {
                None
            }
        })
        .collect()
}

/// Calculates the net aerodynamic drag force from the given surface samples.
///
/// # Parameters
///
/// - `surface_samples`: Slice of `SurfaceSample` structs.
/// - `flow_velocity`: The relative flow velocity vector.
///
/// # Returns
///
/// A `Vec3` representing the net drag force vector.
fn calculate_net_drag(
    surface_samples: &[SurfaceSample],
    flow_velocity: Vec3,
    ambient_density: Density,
) -> Vec3 {
    let dynamic_pressure = 0.5 * ambient_density.0 * flow_velocity.length_squared();

    surface_samples.iter().map(|sample| {
        // Effective area is projected area based on the angle of incidence
        let effective_area = sample.differential_area * flow_velocity.normalize().dot(sample.normal).max(0.0);
        // Drag force contribution from this differential area
        dynamic_pressure * effective_area * sample.normal
    }).sum()
}
