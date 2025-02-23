use avian3d::debug_render::PhysicsDebugPlugin;
use bevy::{
    dev_tools::fps_overlay::FpsOverlayPlugin,
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
};
use big_space::{camera::CameraController, prelude::*};

use crate::colors::ColorPalette;
use buoy_core::prelude::Precision;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsDebugPlugin::default(),               // Draws colliders
        FloatingOriginDebugPlugin::<i64>::default(), // Draws cell AABBs and grids
        WireframePlugin::default(),
        FpsOverlayPlugin::default(),
        TransformPathPlugin,
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
    app.init_resource::<GizmoConfigStore>();
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
    paths: Query<&TransformPath>,
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

    let path_info: Vec<String> = paths.iter()
        .enumerate()
        .map(|(i, path)| {
            format!("Path {}: {} points", i, path.points.len())
        })
        .collect();

    let mut debug_text = debug_text.single_mut();
    debug_text.0.0 = format!(
        "{grid_text}\n{translation_text}\n\n{real_position_f64_text}\n{real_position_f32_text}\n\n{camera_text}\n\nPaths:\n{}",
        path_info.join("\n")
    );
}
pub struct TransformPathPlugin;

impl Plugin for TransformPathPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            add_transform_path,
            update_transform_paths.before(draw_transform_paths),
        ));
    }
}

#[derive(Component, Reflect)]
pub struct TransformPath {
    pub points: Vec<Vec3>,
    pub max_length: usize,
}

impl Default for TransformPath {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            max_length: 1000,
        }
    }
}

fn add_transform_path(
    mut commands: Commands,
    query: Query<Entity, (With<GlobalTransform>, Without<TransformPath>)>,
) {
    for entity in &query {
        commands.entity(entity).insert(TransformPath::default());
    }
}

fn update_transform_paths(
    mut query: Query<(&GlobalTransform, &mut TransformPath)>,
) {
    for (transform, mut path) in &mut query {
        let current_position = transform.translation();
        
        if path.points.last().map_or(true, |last| last.distance(current_position) > 0.01) {
            path.points.push(current_position);
            
            if path.points.len() > path.max_length {
                path.points.remove(0);
            }
        }
    }
}

fn draw_transform_paths(
    mut gizmos: Gizmos,
    query: Query<&TransformPath>,
    mut config_store: ResMut<GizmoConfigStore>,
) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 3.0;
    config.depth_bias = -1.0;
    
    for path in &query {
        if path.points.len() < 2 {
            continue;
        }
        
        for i in 1..path.points.len() {
            let t = i as f32 / path.points.len() as f32;
            let mut color = ColorPalette::BoldPurple.color();
            color.set_alpha(1.0 - t);
            gizmos.line(
                path.points[i - 1],
                path.points[i],
                color,
            );
        }
    }
}
