use std::{collections::VecDeque, time::Duration};

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
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
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, setup_fps_text)
            .add_systems(Update, update_fps_text);
    }
}

#[derive(Component)]
struct FpsText;

fn setup_fps_text(mut commands: Commands) {
    let root_uinode = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        })
        .id();

    let fps_text = commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::End,
            flex_grow: 1.,
            margin: UiRect::axes(Val::Px(15.), Val::Px(5.)),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                Text::default(),
                TextFont {
                    font_size: 10.0,
                    ..Default::default()
                },
                FpsText,
            ));
        })
        .id();

    commands.entity(root_uinode).add_children(&[fps_text]);
}

fn update_fps_text(
    mut fps_history: Local<VecDeque<f64>>,
    mut time_history: Local<VecDeque<Duration>>,
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    query: Query<(Entity, &FpsText)>,
    mut text_query: Query<&mut Text>,
) {
    time_history.push_front(time.elapsed());
    time_history.truncate(120);

    // Safely calculate average FPS
    let avg_fps = match (time_history.front().copied(), time_history.back().copied()) {
        (Some(first), Some(last)) => {
            (time_history.len() as f64) / (first - last).as_secs_f64().max(0.0001)
        }
        _ => 0.0, // Default value if history is empty or if either value is None
    };

    fps_history.push_front(avg_fps);
    fps_history.truncate(120);

    // Safely calculate FPS variance
    let fps_variance = match std_deviation(fps_history.make_contiguous()) {
        Some(variance) => variance,
        None => 0.0, // Default value if std_deviation returns None
    };

    for (entity, _) in &query {
        let fps = match diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            Some(fps_diagnostic) => match fps_diagnostic.smoothed() {
                Some(smoothed_fps) => smoothed_fps,
                None => 0.0,
            },
            None => 0.0,
        };

        let mut frame_time = time.delta_secs_f64();
        if let Some(frame_time_diagnostic) =
            diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            frame_time = match frame_time_diagnostic.smoothed() {
                Some(smoothed_frame_time) => smoothed_frame_time,
                None => frame_time,
            };
        }

        // Update the corresponding Text component directly
        if let Ok(mut text) = text_query.get_mut(entity) {
            text.0 = format!(
                "{frame_time:.3} ms/frame {fps:.1} fps ({avg_fps:.1} avg, {fps_variance:.1} variance)"
            );
        } else {
            eprintln!("No Text component found for entity {:?}", entity);
        }
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
