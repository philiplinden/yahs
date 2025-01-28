use bevy::{
    color::palettes::basic::*,
    prelude::*,
};

use yahs::prelude::{Balloon, Forces, Trajectory, ForceVector};
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

fn force_arrows(forces: Query<&Forces>, mut gizmos: Gizmos) {
    let mut arrows = Vec::new();

    for force in forces.iter().flat_map(|f| &f.vectors) {
        arrows.push(new_force_arrow(force.clone(), force.color.unwrap_or(ColorPalette::VibrantRed.color())));
    }

    for (start, end, color) in arrows {
        gizmos.arrow(start, end, color).with_tip_length(0.3);
    }
}

fn new_force_arrow(force: ForceVector, default_color: Color) -> (Vec3, Vec3, Color) {
    let start = force.point;
    let end = start + force.force * ARROW_SCALE;
    let color = match force.color {
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
        Isometry3d::new(Vec3::new(0.0, height / 2., -1.0), Quat::IDENTITY),
        UVec2::new(width as u32, height as u32),
        Vec2::new(spacing, spacing),
        ColorPalette::MediumBase.color(),
    );
    gizmos.grid(
        Isometry3d::new(Vec3::new(0.0, 0.0, 0.0), Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        UVec2::new(width as u32, height as u32),
        Vec2::new(spacing, spacing),
        ColorPalette::MediumBase.color(),
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
