mod shell;

use bevy::{
    app::PluginGroupBuilder, color::palettes::basic::GREEN, prelude::*,
};
use bevy_egui::EguiPlugin;

/// A plugin group that includes all interface-related plugins
pub struct InterfacePlugins;

impl PluginGroup for InterfacePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CoreUiPlugin)
            // TODO: Add other ui plugins here
    }
}

/// Base UI plugin. This sets up Bevy Egui and the "shell" or frame for the UI.
pub struct CoreUiPlugin;

impl Plugin for CoreUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin,
            shell::ShellPlugin,
        ));
        app.add_systems(Startup, spawn_scene_view);
    }
}

fn spawn_scene_view(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10)),
        material: materials.add(Color::from(GREEN)),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });

    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ));
}
