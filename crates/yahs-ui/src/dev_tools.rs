use std::{collections::VecDeque, time::Duration};

use bevy::{
    prelude::*,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
};

pub struct DevToolsPlugins;

impl Plugin for DevToolsPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(FpsPlugin);
    }
}

struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, setup_fps_text)
            .add_systems(Update, update_fps_text);
    }
}

#[derive(Component)]
struct FpsText;

fn setup_fps_text(mut commands: Commands) {
    let root_uinode = commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    }).id();

    let fps_text = commands.spawn(Node {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::End,
        flex_grow: 1.,
        margin: UiRect::axes(Val::Px(15.), Val::Px(5.)),
        ..default()
    }).with_children(|builder| {
        builder.spawn((
            Text::default(),
            TextFont {
                font_size: 10.0,
                ..Default::default()
            },
            FpsText
        ));
    }).id();

    commands
        .entity(root_uinode)
        .add_children(&[fps_text]);
}

fn update_fps_text(
    mut fps_history: Local<VecDeque<f64>>,
    mut time_history: Local<VecDeque<Duration>>,
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    query: Query<Entity, With<FpsText>>,
    mut writer: TextUiWriter,
) {
    time_history.push_front(time.elapsed());
    time_history.truncate(120);
    
    // Safely calculate average FPS
    let avg_fps = if let (Some(first), Some(last)) = (time_history.front().copied(), time_history.back().copied()) {
        (time_history.len() as f64) / (first - last).as_secs_f64().max(0.0001)
    } else {
        0.0 // Default value if history is empty
    };

    fps_history.push_front(avg_fps);
    fps_history.truncate(120);
    
    // Safely calculate FPS variance
    let fps_variance = std_deviation(fps_history.make_contiguous()).unwrap_or_default();

    for entity in &query {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
                fps = fps_smoothed;
            }
        }

        let mut frame_time = time.delta_secs_f64();
        if let Some(frame_time_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME) {
            if let Some(frame_time_smoothed) = frame_time_diagnostic.smoothed() {
                frame_time = frame_time_smoothed;
            }
        }

        *writer.text(entity, 0) =
            format!("{avg_fps:.1} avg fps, {fps_variance:.1} frametime variance",);

        *writer.text(entity, 1) = format!(
            "\n{fps:.1} fps, {frame_time:.3} ms/frame",
        );

        *writer.text(entity, 4) = format!("{fps:.1}");

        *writer.text(entity, 6) = format!("{frame_time:.3}");
    }
}

fn mean(data: &[f64]) -> Option<f64> {
    let sum = data.iter().sum::<f64>();
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    }
}

fn std_deviation(data: &[f64]) -> Option<f64> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - *value;

                    diff * diff
                })
                .sum::<f64>()
                / count as f64;

            Some(variance.sqrt())
        }
        _ => None,
    }
}
