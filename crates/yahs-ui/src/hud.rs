use bevy::prelude::*;
use avian3d::prelude::*;
use yahs::prelude::{Balloon, Force, Weight, Buoyancy, Drag, IdealGas};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_hud)
            .add_systems(Update, update_hud);
    }
}

#[derive(Component)]
struct HudText;

fn setup_hud(mut commands: Commands) {
    let root_uinode = commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..Default::default()
    }).id();

    let hud_text = commands.spawn(
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Start,
            flex_grow: 1.,
            margin: UiRect::axes(Val::Px(15.), Val::Px(5.)),
            ..Default::default()
        }
    )
    .with_children(|builder| {
        builder.spawn((
            Text::default(),
            TextFont {
                font_size: 10.0,
                ..Default::default()
            },
            HudText
        ));
    }).id();

    commands.entity(root_uinode).add_children(&[hud_text]);
}

fn update_hud(
    time: Res<Time<Physics>>,
    balloons: Query<(&Transform, &Weight, &Buoyancy, &Drag, &IdealGas), With<Balloon>>,
    mut query: Query<&mut Text, With<HudText>>,
) {
    let elapsed_time = time.elapsed_secs();
    let delta_time = time.delta_secs();

    // Safely handle the query for HUD text
    if let Some(mut hud_text) = query.iter_mut().next() {
        let mut text = String::new();

        // Display physics time
        text.push_str(&format!("Physics Time: {:.2} s\n", elapsed_time));

        for (transform, weight, buoyancy, drag, gas) in balloons.iter() {
            text.push_str(&format!("Position: {:} m\n", transform.translation));
            text.push_str(&format!("Velocity: {:?} m/s\n", transform.translation / Vec3::splat(delta_time)));
            text.push_str(&format!("Density: {:.2} kg/m³\n", gas.density.kg_per_m3()));
            text.push_str(&format!("Volume: {:.2} m³\n", gas.volume().m3()));
            text.push_str(&format!("Weight: {:.2} N\n", weight.force().length()));
            text.push_str(&format!("Buoyancy: {:.2} N\n", buoyancy.force().length()));
            text.push_str(&format!("Drag: {:.2} N\n", drag.force().length()));
            text.push_str("\n");
        }

        hud_text.sections[0].value = text; // Update the HUD text
    } else {
        // Handle the case where no HUD text entity is found
        error!("No HUD text entity found. Ensure that the HUD is set up correctly.");
    }
}
