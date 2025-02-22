use bevy::{
    prelude::*,
    pbr::wireframe::{WireframeConfig, WireframePlugin},
};
use avian3d::debug_render::PhysicsDebugPlugin;
use big_space::{prelude::*, camera::CameraController};

use buoy_core::prelude::Precision;
use crate::colors::ColorPalette;


pub(crate) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsDebugPlugin::default(), // Draws colliders
        FloatingOriginDebugPlugin::<i64>::default(), // Draws cell AABBs and grids
        WireframePlugin::default(),
    ));
    app.add_systems(Startup, ui_setup);
    app.add_systems(PreUpdate, ui_text_system);
    app.insert_resource(WireframeConfig {
        // The global wireframe config enables drawing of wireframes on every mesh,
        // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
        // regardless of the global configuration.
        global: true,
        // Controls the default color of all wireframes. Used as the default color for global wireframes.
        // Can be changed per mesh using the `WireframeColor` component.
        default_color: ColorPalette::LightBase.color(),
    });
}

#[derive(Component, Reflect)]
pub struct BigSpaceDebugText;

fn ui_setup(mut commands: Commands) {
    commands.spawn((
        Text::default(),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Left),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        BigSpaceDebugText,
    ));
}

#[allow(clippy::type_complexity)]
fn ui_text_system(
    mut debug_text: Query<(&mut Text, &GlobalTransform), With<BigSpaceDebugText>>,
    grids: Grids<Precision>,
    time: Res<Time>,
    origin: Query<(Entity, GridTransformReadOnly<Precision>), With<FloatingOrigin>>,
    camera: Query<&CameraController>,
) {
    let (origin_entity, origin_pos) = origin.single();
    let translation = origin_pos.transform.translation;

    let grid_text = format!(
        "GridCell: {}x, {}y, {}z",
        origin_pos.cell.x, origin_pos.cell.y, origin_pos.cell.z
    );

    let translation_text = format!(
        "Transform: {}x, {}y, {}z",
        translation.x, translation.y, translation.z
    );

    let Some(grid) = grids.parent_grid(origin_entity) else {
        return;
    };

    let real_position = grid.grid_position_double(origin_pos.cell, origin_pos.transform);
    let real_position_f64_text = format!(
        "Combined (f64): {}x, {}y, {}z",
        real_position.x, real_position.y, real_position.z
    );
    let real_position_f32_text = format!(
        "Combined (f32): {}x, {}y, {}z",
        real_position.x as f32, real_position.y as f32, real_position.z as f32
    );

    let velocity = camera.single().velocity();
    let speed = velocity.0.length() / time.delta_secs_f64();
    let camera_text = if speed > 3.0e8 {
        format!("Speed: {:.0e} * speed of light", speed / 3.0e8)
    } else {
        format!("Speed: {:.2e} m/s", speed)
    };

    let mut debug_text = debug_text.single_mut();

    debug_text.0.0 = format!(
        "{grid_text}\n{translation_text}\n\n{real_position_f64_text}\n{real_position_f32_text}\n\n{camera_text}"
    );
}
