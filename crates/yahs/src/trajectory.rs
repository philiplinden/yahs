use std::time::Duration;
use bevy::prelude::*;

use crate::prelude::SimState;

const DEFAULT_UPDATE_INTERVAL: Duration = Duration::from_millis(100);

pub struct TrajectoryPlugin;

impl Plugin for TrajectoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_trajectory.run_if(in_state(SimState::Running)),
        );
    }
}

#[derive(Component)]
pub struct Trajectory {
    pub points: Vec<Vec3>,
    pub timer: Timer,
}

impl Trajectory {
    pub fn new(update_interval: Duration) -> Self {
        Self {
            points: Vec::new(),
            timer: Timer::new(update_interval, TimerMode::Repeating),
        }
    }

    /// Updates the update interval of the trajectory.
    pub fn change_interval(&mut self, new_interval: Duration) {
        self.timer.set_duration(new_interval);
    }

    /// Adds a point to the trajectory.
    pub fn add_point(&mut self, point: Vec3) {
        self.points.push(point);
    }

    /// Resets the trajectory by clearing all points.
    pub fn reset(&mut self) {
        self.points.clear();
    }
}

impl Default for Trajectory {
    fn default() -> Self {
        Self::new(DEFAULT_UPDATE_INTERVAL)
    }
}

fn update_trajectory(
    time: Res<Time>,
    mut query: Query<(&mut Trajectory, &Transform)>,
) {
    for (mut trajectory, transform) in query.iter_mut() {
        trajectory.timer.tick(time.delta());
        if trajectory.timer.just_finished() {
            trajectory.points.push(transform.translation.clone());
            debug!("Added point to trajectory: {:?}", transform.translation);
        }
    }
}
