use avian3d::prelude::*;
use bevy::prelude::*;
use yahs::prelude::{Balloon, Buoyancy, Drag, Force, Volume, Density, SimState, Weight};
use crate::controls::KeyBindingsConfig;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hud)
            .add_systems(Update, (update_hud, update_settings_text));
    }
}

#[derive(Component)]
struct KinematicsText;

#[derive(Component)]
struct SettingsText;

fn setup_hud(mut commands: Commands) {
    let root_uinode = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..Default::default()
        })
        .id();

    let hud_text = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Start,
            margin: UiRect::axes(Val::Px(15.), Val::Px(5.)),
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn((
                Text::default(),
                TextFont {
                    font_size: 10.0,
                    ..Default::default()
                },
                KinematicsText,
            ));
            builder.spawn((
                Text::default(),
                TextFont {
                    font_size: 10.0,
                    ..Default::default()
                },
                SettingsText,
            ));
        })
        .id();

    commands.entity(root_uinode).add_children(&[hud_text]);
}

fn update_hud(
    time: Res<Time<Physics>>,
    state: Res<State<SimState>>,
    balloons: Query<
        (
            &Name,
            &Transform,
            &Weight,
            &Buoyancy,
            &Drag,
            &ExternalForce,
            &LinearVelocity,
            &Volume,
            &Density,
        ),
        With<Balloon>,
    >,
    mut query: Query<&mut Text, With<KinematicsText>>,
) {
    let elapsed_time = time.elapsed_secs();

    // Safely handle the query for HUD text
    if let Ok(mut kinematics_text) = query.get_single_mut() {
        let mut text = String::new();

        // Display physics time
        text.push_str(&format!("Sim State: {:?}\n", state.get()));
        text.push_str(&format!("Physics Time: {:.2} s\n", elapsed_time));

        for (name, transform, weight, buoyancy, drag, net_force, velocity, volume, density) in balloons.iter() {
            text.push_str(&format!("\n{}\n", name.as_str()));
            text.push_str(&format!("Position: {:} m\n", transform.translation));
            text.push_str(&format!("Velocity: {:?} m/s\n", velocity.0));
            text.push_str(&format!("Density: {:.2} kg/m3\n", density.kg_per_m3()));
            text.push_str(&format!("Volume: {:.2} m3\n", volume.m3()));
            text.push_str(&format!("Weight: {:.2} N\n", weight.force()));
            text.push_str(&format!("Buoyancy: {:.2} N\n", buoyancy.force()));
            text.push_str(&format!("Drag: {:.2} N\n", drag.force()));
            text.push_str(&format!("Net Force: {:.2} N\n", net_force.force()));
            text.push_str("\n");
        }

        kinematics_text.0 = text; // Update the HUD text
    } else {
        // Handle the case where no HUD text entity is found
        error!("No HUD text entity found. Ensure that the HUD is set up correctly.");
    }
}

fn update_settings_text(
    key_bindings: Res<KeyBindingsConfig>,
    mut query: Query<&mut Text, With<SettingsText>>,
) {
    if let Ok(mut settings_text) = query.get_single_mut() {
        let mut text = String::new();

        // Display keybindings
        text.push_str(&format!("\n\nToggle Pause: {:?}\n", key_bindings.time_controls.toggle_pause));
        text.push_str(&format!("Faster: {:?}\n", key_bindings.time_controls.faster));
        text.push_str(&format!("Slower: {:?}\n", key_bindings.time_controls.slower));
        text.push_str(&format!("Reset Speed: {:?}\n", key_bindings.time_controls.reset_speed));
        text.push_str(&format!("Step Once: {:?}\n", key_bindings.time_controls.step_once));

        settings_text.0 = text; // Update the settings text
    } else {
        error!("No settings text entity found. Ensure that the settings text is set up correctly.");
    }
}
