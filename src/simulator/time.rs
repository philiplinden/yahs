use avian3d::prelude::*;
use bevy::prelude::*;

use super::SimState;

pub struct TimeScalePlugin;

impl Plugin for TimeScalePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeScaleOptions>();
        app.add_systems(
            OnEnter(SimState::Stopped),
            pause,
        );
        app.add_systems(
            OnExit(SimState::Stopped),
            unpause,
        );
        app.add_systems(
            PreUpdate,
            modify_time_scale.run_if(in_state(SimState::Running)),
        );
    }
}

#[derive(Resource)]
pub struct TimeScaleOptions {
    pub multiplier: f32,
}

impl Default for TimeScaleOptions {
    fn default() -> Self {
        Self { multiplier: 1.0 }
    }
}

fn modify_time_scale(
    mut time: ResMut<Time<Physics>>,
    options: Res<TimeScaleOptions>,
) {
    if options.is_changed() {
        info!("setting relative speed to {}", options.multiplier);
        time.as_mut().set_relative_speed(options.multiplier);
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
