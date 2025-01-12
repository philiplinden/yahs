mod colors;
mod controls;
mod ui;

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_ratatui::terminal::RatatuiContext;
use ratatui::prelude::*;
use yahs::prelude::*;

pub struct TuiPlugin;

impl Plugin for TuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(controls::ControlsPlugin)
            .add_systems(Startup, spawn_balloon)
            .add_systems(Update, draw_tui.pipe(bevy_ratatui::error::exit_on_error));
    }
}

fn draw_tui(
    mut context: ResMut<RatatuiContext>,
    time: Res<Time<Physics>>,
    state: Res<State<SimState>>,
    time_options: Res<TimeScaleOptions>,
    balloons: Query<
        (
            &Name,
            &Transform,
            &Weight,
            &Buoyancy,
            &yahs::prelude::Drag,
            &IdealGas,
        ),
        With<Balloon>,
    >,
) -> color_eyre::Result<()> {
    context.draw(|frame| {
        let area = frame.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Length(3), // Sim Status
                Constraint::Min(10),   // Balloon Data
                Constraint::Length(3), // Controls
            ])
            .split(area);

        ui::draw_title(frame, chunks[0]);
        ui::draw_status(
            frame,
            chunks[1],
            state.get(),
            time.elapsed_secs(),
            &time_options,
        );
        ui::draw_balloon_data(frame, chunks[2], &balloons.iter().collect::<Vec<_>>());
        ui::draw_controls(frame, chunks[3]);
    })?;
    Ok(())
}
