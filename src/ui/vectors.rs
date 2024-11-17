use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_prototype_debug_lines::*;

use crate::simulator::forces::Force;

pub struct ForceVectorPlugin;

impl Plugin for ForceVectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DebugLines::default());
        app.add_systems(Update, visualize_forces);
    }
}

/// System to visualize all force vectors using DebugLines
fn visualize_forces(
    query: Query<(&Transform, &dyn Force), With<RigidBody>>,
    mut debug_lines: ResMut<DebugLines>,
) {
    for (transform, force) in query.iter() {
        let origin = force.point_of_application().unwrap_or(transform.translation);
        let force_vector = force.force();

        // Define the color based on force type or magnitude if needed
        let color = match force.downcast_ref::<crate::simulator::forces::Weight>() {
            Some(_) => Color::RED,
            None => match force.downcast_ref::<crate::simulator::forces::Buoyancy>() {
                Some(_) => Color::GREEN,
                None => Color::BLUE,
            },
        };

        // Draw the force vector
        debug_lines.line(origin, origin + force_vector, color, 0.0);
    }
} 
