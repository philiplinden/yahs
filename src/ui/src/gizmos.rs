use bevy::{color::palettes::basic::*, prelude::*};

use crate::simulator::forces::Force;

const ARROW_SCALE: f32 = 0.1;

pub struct ForceArrowsPlugin;

impl Plugin for ForceArrowsPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<ForceGizmos>();
        app.register_type::<ForceGizmos>();
        app.add_systems(
            PostUpdate,
            force_arrows.run_if(
                |store: Res<GizmoConfigStore>| {
                    store.config::<ForceGizmos>().0.enabled
                }),
        );
    }
}

fn force_arrows(
    query: Query<&dyn Force>,
    mut gizmos: Gizmos,
) {
    for forces in query.iter() {
        for force in forces.iter() {
            let start = force.point_of_application();
            let end = start + force.force() * ARROW_SCALE;
            let color = match force.color() {
                Some(c) => c,
                None => RED.into(),
            };
            gizmos.arrow(start, end, color).with_tip_length(0.3);
        }
    }
}

#[derive(Reflect, GizmoConfigGroup)]
pub struct ForceGizmos {
    /// The scale of the force arrows.
    pub arrow_scale: Option<f32>,
    /// The color of the force arrows. If `None`, the arrows will not be rendered.
    pub arrow_color: Option<Color>,
    /// The length of the arrow tips.
    pub tip_length: Option<f32>,
    /// Determines if the forces should be hidden when not active.
    pub enabled: bool,
}

impl Default for ForceGizmos {
    fn default() -> Self {
        Self {
            arrow_scale: Some(0.1),
            arrow_color: Some(RED.into()),
            tip_length: Some(0.3),
            enabled: false,
        }
    }
}

impl ForceGizmos {
    /// Creates a [`ForceGizmos`] configuration with all rendering options enabled.
    pub fn all() -> Self {
        Self {
            arrow_scale: Some(0.1),
            arrow_color: Some(RED.into()),
            tip_length: Some(0.3),
            enabled: true,
        }
    }

    /// Creates a [`ForceGizmos`] configuration with debug rendering enabled but all options turned off.
    pub fn none() -> Self {
        Self {
            arrow_scale: None,
            arrow_color: None,
            tip_length: None,
            enabled: false,
        }
    }
}
