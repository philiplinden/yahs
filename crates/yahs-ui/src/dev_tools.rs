use bevy::{
    color::palettes::basic::*,
    prelude::*,
};

use yahs::prelude::{Balloon, Force, Weight, Buoyancy, Drag};

pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            KinematicsGizmos,
        ));
    }
}

const ARROW_SCALE: f32 = 0.1;

pub struct KinematicsGizmos;

impl Plugin for KinematicsGizmos {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (force_arrows, orientation_indicator, position_reference)
        );
    }
}

fn force_arrows(weights: Query<&Weight>, buoys: Query<&Buoyancy>, drags: Query<&Drag>, mut gizmos: Gizmos) {
    for weight in weights.iter() {
        let start = weight.point_of_application();
        let end = start + weight.force() * ARROW_SCALE;
        let color = match weight.color() {
            Some(c) => c,
                None => RED.into(),
            };
        gizmos.arrow(start, end, color).with_tip_length(0.3);
    }
    for buoyancy in buoys.iter() {
        let start = buoyancy.point_of_application();
        let end = start + buoyancy.force() * ARROW_SCALE;
        
        let color = match buoyancy.color() {
            Some(c) => c,
                None => BLUE.into(),
            };
        gizmos.arrow(start, end, color).with_tip_length(0.3);
    }
    for drag in drags.iter() {
        let start = drag.point_of_application();
        let end = start + drag.force() * ARROW_SCALE;
        let color = match drag.color() {
            Some(c) => c,
                None => GREEN.into(),
            };
        gizmos.arrow(start, end, color).with_tip_length(0.3);
    }
}

fn orientation_indicator(query: Query<&Transform, With<Balloon>>, mut gizmos: Gizmos) {
    for transform in query.iter() {
        gizmos.cross(transform.translation, 2.0, LIME);
    }
}

/// A grid pinned to the world frame of reference. It is set back a little bit
/// so it doesn't z-fight with other gizmos.
fn position_reference(mut gizmos: Gizmos) {
    gizmos.grid(
        Isometry3d::new(Vec3::new(0.0, 0.0, -1.0), Quat::IDENTITY),
        UVec2::splat(20),
        Vec2::new(2., 2.),
        LinearRgba::gray(0.65),
    );
}
