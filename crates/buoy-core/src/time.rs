use avian3d::prelude::*;
use bevy::prelude::*;

use crate::core::SimState;

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<TimeScaleOptions>();
    app.init_resource::<TimeScaleOptions>();
    app.add_event::<StepPhysicsEvent>();
    app.add_systems(OnEnter(SimState::Stopped), pause);
    app.add_systems(OnExit(SimState::Stopped), unpause);
    app.add_systems(PreUpdate, modify_time_scale);
    app.add_systems(Update, step_physics_once_on_event);
}

const DEFAULT_MULTIPLIER: f32 = 100.0;

#[derive(Resource)]
pub struct TimeScaleOptions {
    pub multiplier: f32,
    pub max_multiplier: f32,
    pub min_multiplier: f32,
}

impl Default for TimeScaleOptions {
    fn default() -> Self {
        Self {
            multiplier: DEFAULT_MULTIPLIER,
            max_multiplier: 10.0,
            min_multiplier: 0.1,
        }
    }
}

impl TimeScaleOptions {
    pub fn reset(&mut self) {
        self.multiplier = DEFAULT_MULTIPLIER;
    }
}

fn modify_time_scale(mut time: ResMut<Time<Physics>>, options: Res<TimeScaleOptions>) {
    if options.is_changed() {
        // We use this toggle with a bool so we can use the same key to toggle
        // between real time and previous multiplier.
        time.as_mut().set_relative_speed(
            options
                .multiplier
                .clamp(options.min_multiplier, options.max_multiplier),
        );
    }
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

#[derive(Event)]
pub struct StepPhysicsEvent(pub f32);

fn step_physics_once_on_event(
    mut events: EventReader<StepPhysicsEvent>,
    mut physics_time: ResMut<Time<Physics>>,
    mut next_state: ResMut<NextState<SimState>>,
) {
    for event in events.read() {
        let t = physics_time.as_mut();
        let delta = std::time::Duration::from_secs_f32(event.0);
        debug!("stepping physics time by {:?}", delta);
        t.advance_by(delta);
        next_state.set(SimState::Stopped);
    }
}
