use avian3d::prelude::*;
use bevy::prelude::*;
use yahs::prelude::{Balloon, Forces, Volume, Density, SimState, IdealGas};
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
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::Start,
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
            Entity,
            &Transform,
            &Forces,
            &LinearVelocity,
            &Children,
        ),
        With<Balloon>,
    >,
    mut query: Query<&mut Text, With<KinematicsText>>,
    children_forces: Query<&Forces, With<Parent>>,
    gases: Query<(&Volume, &Density), With<IdealGas>>,
) {
    let elapsed_time = time.elapsed_secs();

    // Safely handle the query for HUD text
    if let Ok(mut kinematics_text) = query.get_single_mut() {
        let mut text = String::new();

        // Display physics time
        text.push_str(&format!("Sim State: {:?}\n", state.get()));
        text.push_str(&format!("Physics Time: {:.4} s\n", elapsed_time));

        for (entity, transform, forces, velocity, children) in balloons.iter() {
            text.push_str(&format!("\n{:?}\n", entity));
            text.push_str(&format!("Position: {:.4} m\n", transform.translation));
            text.push_str(&format!("Velocity: {:.4} m/s\n", velocity.0));
            
            let mut num_forces = forces.vectors.len();
            let mut total_force = forces.net_force();
            for &child in children.iter() {
                if let Ok(child_forces) = children_forces.get(child) {
                    total_force += child_forces.net_force();
                    num_forces += child_forces.vectors.len();
                }
                if let Ok((volume, density)) = gases.get(child) {   
                    text.push_str(&format!("Density: {:.4} kg/m3\n", density.kg_per_m3()));
                    text.push_str(&format!("Volume: {:.4} m3\n", volume.m3()));
                }
            }
            text.push_str(&format!("Forces: {:.4} N from {:?} forces", total_force.force, num_forces));
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
        text.push_str(&format!("Rotate Camera: {:?} Mouse\n", key_bindings.camera_controls.hold_look));
        settings_text.0 = text; // Update the settings text
    } else {
        error!("No settings text entity found. Ensure that the settings text is set up correctly.");
    }
}
