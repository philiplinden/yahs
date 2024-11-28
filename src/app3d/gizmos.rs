use bevy::{color::palettes::basic::*, prelude::*};

use crate::simulator::{forces::Force, SimState};

const ARROW_SCALE: f32 = 0.1;

pub struct ForceArrowsPlugin;

impl Plugin for ForceArrowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, force_arrows);
    }
}

fn force_arrows(query: Query<&dyn Force>, mut gizmos: Gizmos) {
    for forces in query.iter() {
        for force in forces.iter() {
            let start = force.point_of_application();
            let end = start + force.force() * ARROW_SCALE;
            let color = force.color().unwrap_or(RED.into());
            gizmos.arrow(start, end, color).with_tip_length(0.1);
        }
    }
}
