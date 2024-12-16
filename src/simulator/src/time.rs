use avian3d::prelude::*;
use bevy::prelude::*;

use crate::core::SimState;

pub struct TimeScalePlugin;

impl Plugin for TimeScalePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeScaleOptions>();
        app.add_systems(OnEnter(SimState::Stopped), pause);
        app.add_systems(OnExit(SimState::Stopped), unpause);
        app.add_systems(
            PreUpdate,
            modify_time_scale.run_if(in_state(SimState::Running)),
        );
    }
}

const DEFAULT_MULTIPLIER: f32 = 2.0;

#[derive(Resource)]
pub struct TimeScaleOptions {
    pub multiplier: f32,
    pub real_time: bool,
    pub max_multiplier: f32,
    pub min_multiplier: f32,
}

impl Default for TimeScaleOptions {
    fn default() -> Self {
        Self {
            multiplier: DEFAULT_MULTIPLIER,
            real_time: false,
            max_multiplier: 3.0,
            min_multiplier: 0.1,
        }
    }
}

impl TimeScaleOptions {
    pub fn reset(&mut self) {
        self.multiplier = DEFAULT_MULTIPLIER;
        self.real_time = false;
    }

    pub fn toggle_real_time(&mut self) {
        self.real_time = !self.real_time;
    }
}

fn modify_time_scale(mut time: ResMut<Time<Physics>>, options: Res<TimeScaleOptions>) {
    if options.is_changed() {
        info!("setting relative speed to {}", options.multiplier);
        if options.real_time {
            time.as_mut().set_relative_speed(1.0);
        } else {
            time.as_mut().set_relative_speed(
                options
                    .multiplier
                    .clamp(options.min_multiplier, options.max_multiplier),
            );
        }
    }
}

fn pause(mut physics_time: ResMut<Time<Physics>>, mut virtual_time: ResMut<Time<Virtual>>) {
    physics_time.as_mut().pause();
    virtual_time.as_mut().pause();
}

fn unpause(mut physics_time: ResMut<Time<Physics>>, mut virtual_time: ResMut<Time<Virtual>>) {
    physics_time.as_mut().unpause();
    virtual_time.as_mut().unpause();
}
