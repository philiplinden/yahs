use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use super::controls::KeyBindingsConfig;
use crate::simulator::Balloon;

const INVERT_ZOOM: bool = true;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraSelection>();
        app.add_systems(Startup, setup);
        app.add_systems(Update, zoom_camera);
        app.add_plugins(CameraFollowPlugin);
    }
}

#[derive(Component, Default)]
#[require(Camera3d, PerspectiveProjection)]
struct MainCamera;

/// A resource that stores the currently selected camera target.
#[derive(Resource)]
struct CameraSelection {
    entity: Entity,
    offset: Vec3,
}

impl Default for CameraSelection {
    fn default() -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
            offset: Vec3::new(0., 0., 10.),
        }
    }
}

/// A marker component for entities that can be selected as a camera target.
#[derive(Component, Default, Reflect)]
pub struct CameraTarget;

fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("Main Camera"),
        MainCamera,
        Camera3d::default(),
        Transform::from_xyz(0.0, 20., 50.0).looking_at(Vec3::new(0., 20., 0.), Vec3::Y),
    ));
}

fn zoom_camera(
    mut camera: Query<&mut PerspectiveProjection, (With<Camera3d>, With<MainCamera>)>,
    mut evr_scroll: EventReader<MouseWheel>,
    key_bindings: Res<KeyBindingsConfig>,
) {
    let mut projection = camera.single_mut();
    let ctrl = &key_bindings.camera_controls;
    let direction = if INVERT_ZOOM { -1.0 } else { 1.0 };
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                projection.fov = projection.fov.clamp(ctrl.min_fov, ctrl.max_fov)
                    + ev.y * ctrl.zoom_step * direction;
            }
            MouseScrollUnit::Pixel => {
                projection.fov = projection.fov.clamp(ctrl.min_fov, ctrl.max_fov)
                    + ev.y * ctrl.zoom_step * direction;
            }
        }
    }
}

struct CameraFollowPlugin;

impl Plugin for CameraFollowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (mark_new_targets, follow_selected_target));
    }
}

fn mark_new_targets(
    mut commands: Commands,
    balloons: Query<Entity, Added<Balloon>>,
    mut selection: ResMut<CameraSelection>,
) {
    for entity in &balloons {
        commands.entity(entity).insert(CameraTarget);
        // Focus on the newest balloon
        selection.entity = entity;
    }
}

fn follow_selected_target(
    selection: Res<CameraSelection>,
    targets: Query<&Transform, (With<CameraTarget>, Without<MainCamera>)>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
) {
    let mut cam = camera.single_mut();
    match targets.get(selection.entity) {
        Ok(t) => {
            // If the target exists, move the camera next to it
            cam.translation = t.translation + selection.offset;
            // Look at the target position
            cam.look_at(t.translation, Vec3::Y);
        }
        Err(_) => {
            // If there is no selected entity, stay where you are
        }
    }
}
