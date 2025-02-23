use avian3d::prelude::*;
use bevy::prelude::*;

use crate::core::SimState;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(SimState::Stopped), pause);
    app.add_systems(OnExit(SimState::Stopped), unpause);
}

pub fn pause(mut physics_time: ResMut<Time<Physics>>, mut next_state: ResMut<NextState<SimState>>) {
    physics_time.as_mut().pause();
    debug!("pausing physics time");
    next_state.set(SimState::Stopped);
}

pub fn unpause(
    mut physics_time: ResMut<Time<Physics>>,
    mut next_state: ResMut<NextState<SimState>>,
) {
    physics_time.as_mut().unpause();
    debug!("unpausing physics time");
    next_state.set(SimState::Running);
}
