use avian3d::{math::{FRAC_PI_2, TAU}, prelude::*};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

/// Calculates drag force vector and parameters for a given collider and velocity
pub fn aero_drag_from_collider(
    collider: &Collider,
    velocity: Vec3,
    wind_velocity: Vec3,
) -> Vec3 {
    // Calculate relative flow velocity
    let flow_velocity = velocity - wind_velocity;
    let view_vector = flow_velocity; 

    let surface_points: Vec<(Vec3, Vec3)> = sample_surface(collider, 100, view_vector);
    calculate_net_drag(&surface_points, flow_velocity, 1.225)
}

/// Samples points on the collider's surface based on the view vector. Samples
/// are concentrated around the view vector direction.
/// 
/// For aerodynamic calculations, we only care about the surface interactions.
/// Setting `solid = false` ensures we always get points on the surface, even if
/// our sample point would be inside the collider. would be inside the collider.
fn sample_surface(collider: &Collider, samples: usize, view_vector: Vec3) -> Vec<(Vec3, Vec3)> {
    let mut surface_points = Vec::with_capacity(samples);
    let mut rng = thread_rng();

    // Normalize the view vector to get the sampling direction
    let view_dir = view_vector.normalize();

    for _ in 0..samples {
        // Generate a random point on a hemisphere oriented along the view direction
        let u: f32 = rng.gen();
        let v: f32 = rng.gen();

        let theta = u * FRAC_PI_2; // Hemisphere: 0 to π/2
        let phi = v * TAU;

        // Spherical coordinates to Cartesian coordinates
        let x = theta.sin() * phi.cos();
        let y = theta.sin() * phi.sin();
        let z = theta.cos();

        // Create a direction vector in the hemisphere
        let dir = Vec3::new(x, y, z);

        // Rotate the direction to align with the view direction
        let rotation = Quat::from_rotation_arc(Vec3::Z, view_dir);
        let sampled_dir = rotation * dir;

        // Project the sampled direction onto the collider's surface
        let far_point = sampled_dir * 1000.0; // Far point in the sampled direction
        let (projected_point, normal_inside) = collider.project_point(
            Vec3::ZERO,        // translation
            Quat::IDENTITY,    // rotation
            far_point,
            false, // treat as hollow
        );

        // Ensure the normal is correctly oriented
        let normal = (projected_point - Vec3::ZERO).normalize();
        surface_points.push((projected_point, normal));
    }

    surface_points
}

fn calculate_net_drag(
    surface_points: &[(Vec3, Vec3)],
    flow_velocity: Vec3,
    air_density: f32,
) -> (f32, Vec3) {
    let mut total_force = Vec3::ZERO;
    let mut total_area = 0.0;

    for (position, normal) in surface_points {
        let dot = normal.dot(flow_velocity.normalize()).max(0.0); // Projection
        let area = 1.0; // Approximation for now; replace with actual differential area
        let cp = 2.0 * dot.powi(2); // Pressure coefficient

        let local_force = 0.5 * air_density * flow_velocity.length_squared() * cp * area;
        total_force += local_force * *normal;
        total_area += area;
    }
    (total_area, total_force)
}
