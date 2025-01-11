use bevy::{
    color::palettes::basic::*,
    prelude::*,
};

use yahs::prelude::{Balloon, Force, Weight, Buoyancy, Drag, Trajectory};
use crate::colors::ColorPalette;

const ARROW_SCALE: f32 = 0.1;

pub struct KinematicsGizmos;

impl Plugin for KinematicsGizmos {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (force_arrows, orientation_indicator, position_reference, draw_trajectory)
        );
    }
}

fn force_arrows(weights: Query<&Weight>, buoys: Query<&Buoyancy>, drags: Query<&Drag>, mut gizmos: Gizmos) {
    let mut arrows = Vec::new();

    for weight in weights.iter() {
        arrows.push(new_force_arrow(weight, ColorPalette::VibrantRed.color()));
    }

    for buoyancy in buoys.iter() {
        arrows.push(new_force_arrow(buoyancy, ColorPalette::BrightBlue.color()));
    }

    for drag in drags.iter() {
        arrows.push(new_force_arrow(drag, ColorPalette::LivelyGreen.color()));
    }

    for (start, end, color) in arrows {
        gizmos.arrow(start, end, color).with_tip_length(0.3);
    }
}

fn new_force_arrow(force: &dyn Force, default_color: Color) -> (Vec3, Vec3, Color) {
    let start = force.point_of_application();
    let end = start + force.force() * ARROW_SCALE;
    let color = match force.color() {
        Some(c) => c,
        None => default_color,
    };
    (start, end, color)
}

fn orientation_indicator(query: Query<&Transform, With<Balloon>>, mut gizmos: Gizmos) {
    for transform in query.iter() {
        gizmos.cross(transform.translation, 2.0, LIME);
    }
}

/// A grid pinned to the world frame of reference. It is set back a little bit
/// so it doesn't z-fight with other gizmos.
fn position_reference(mut gizmos: Gizmos) {
    let height = 100.;
    let width = 20.;
    let spacing = 1.0;
    gizmos.grid(
        Isometry3d::new(Vec3::new(0.0, height / 2. - 10., -1.0), Quat::IDENTITY),
        UVec2::new(width as u32, height as u32),
        Vec2::new(spacing, spacing),
        ColorPalette::LightBase.color(),
    );
}

/// Draws the trajectory of the balloon by connecting points in the trajectory.
fn draw_trajectory(query: Query<(&Trajectory, &Transform)>, mut gizmos: Gizmos) {
    for (trajectory, transform) in query.iter() {
        let mut start = transform.translation;
        for point in &trajectory.points {
            gizmos.line(start, *point, ColorPalette::VividYellow.color());
            start = *point;
        }
    }
}
