use bevy::prelude::*;
use bevy_ratatui::event::KeyEvent;
use yahs::prelude::*;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_input_system);
    }
}

pub fn keyboard_input_system(
    mut events: EventReader<KeyEvent>,
    mut exit: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<SimState>>,
    mut time_options: ResMut<TimeScaleOptions>,
    state: Res<State<SimState>>,
) {
    use crossterm::event::KeyCode;
    for event in events.read() {
        match event.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                exit.send_default();
            }
            KeyCode::Char(' ') => match state.get() {
                SimState::Running => next_state.set(SimState::Stopped),
                SimState::Stopped => next_state.set(SimState::Running),
                _ => {}
            },
            KeyCode::Char('r') => {
                time_options.reset();
            }
            KeyCode::Left => {
                time_options.multiplier =
                    (time_options.multiplier / 2.0).max(time_options.min_multiplier);
            }
            KeyCode::Right => {
                time_options.multiplier =
                    (time_options.multiplier * 2.0).min(time_options.max_multiplier);
            }
            _ => {}
        }
    }
}
