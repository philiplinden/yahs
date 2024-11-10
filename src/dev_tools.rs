//! Development tools for the game. This plugin is only enabled in dev builds.
#[allow(unused_imports)]
use bevy::{
    color::palettes::basic::SILVER,
    dev_tools::{
        states::log_transitions,
        ui_debug_overlay::{DebugUiPlugin, UiDebugOptions},
    },
    input::common_conditions::input_just_pressed,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};

#[cfg(not(target_arch = "wasm32"))]
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};

const TOGGLE_DEBUG_UI_KEY: KeyCode = KeyCode::F3;
const TOGGLE_WIREFRAME_KEY: KeyCode = KeyCode::F4;

pub(super) fn plugin(app: &mut App) {
    // Toggle the debug overlay for UI.
    app.add_plugins((
        DebugUiPlugin,
        #[cfg(not(target_arch = "wasm32"))]
        WireframePlugin,
    ));
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_DEBUG_UI_KEY)),
    );
    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(
        Update,
        toggle_wireframe.run_if(input_just_pressed(TOGGLE_WIREFRAME_KEY)),
    );
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

#[cfg(not(target_arch = "wasm32"))]
fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(TOGGLE_WIREFRAME_KEY) {
        wireframe_config.global = !wireframe_config.global;
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}
