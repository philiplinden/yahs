#[allow(unused_imports)]
use bevy::{
    color::palettes::basic::*,
    dev_tools::{states::log_transitions, fps_overlay::{FpsOverlayPlugin, FpsOverlayConfig}},
    diagnostic::*,
    input::common_conditions::input_just_pressed,
    prelude::*,
    time::common_conditions::on_timer,
    
    text::FontSmoothing,
};
use avian3d::debug_render::*;

use super::controls::KeyBindingsConfig;
use yahs_simulator::prelude::{Force, SimState};

// Define all Show and Hide events
#[derive(Event)]
struct ShowEngineDebug;

#[derive(Event)]
struct HideEngineDebug;

#[derive(Event)]
struct ShowWireframe;

#[derive(Event)]
struct HideWireframe;

#[derive(Event)]
struct ShowForceGizmos;

#[derive(Event)]
struct HideForceGizmos;

#[derive(Event)]
struct ShowPhysicsGizmos;

#[derive(Event)]
struct HidePhysicsGizmos;

pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
            RenderedDevToolsPlugin,
        ));

        app.add_systems(Update, log_transitions::<SimState>);
    }
}

struct RenderedDevToolsPlugin;

impl Plugin for RenderedDevToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsDebugPlugin::default(),
            ForceArrowsPlugin,
            #[cfg(not(target_arch = "wasm32"))]
            bevy::pbr::wireframe::WireframePlugin,
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        // Here we define size of our overlay
                        font_size: 14.0,
                        // If we want, we can use a custom font
                        font: default(),
                        // We could also disable font smoothing,
                        font_smoothing: FontSmoothing::default(),
                    },
                    // We can also change color of the overlay
                    text_color: Color::srgb(0.0, 1.0, 0.0),
                    enabled: false,
                },
            },
        ));

        app.init_resource::<DebugState>();

        // Register all events
        app.add_event::<ShowEngineDebug>();
        app.add_event::<HideEngineDebug>();
        app.add_event::<ShowForceGizmos>();
        app.add_event::<HideForceGizmos>();
        app.add_event::<ShowPhysicsGizmos>();
        app.add_event::<HidePhysicsGizmos>();

        app.add_systems(Startup, init_debug_ui);
        app.add_systems(Update, trigger_debug_ui);

        // Register handler functions as observers
        app.add_observer(show_engine_debug);
        app.add_observer(show_force_gizmos);
        app.add_observer(show_physics_gizmos);
        #[cfg(not(target_arch = "wasm32"))]
        {
            app.add_event::<ShowWireframe>();
            app.add_event::<HideWireframe>();
            app.add_observer(show_wireframe);
            app.add_observer(hide_wireframe);
        }
        
        app.add_observer(hide_engine_debug);
        app.add_observer(hide_force_gizmos);
        app.add_observer(hide_physics_gizmos);

        #[cfg(feature = "inspect")]
        {
            use bevy_inspector_egui::quick;
            app.add_plugins((
                quick::FilterQueryInspectorPlugin::default(),
                quick::ResourceInspectorPlugin::default(),
                quick::StateInspectorPlugin::default(),
                quick::WorldInspectorPlugin::default(),
                quick::AssetInspectorPlugin::<Mesh3d>::default(),
                quick::AssetInspectorPlugin::<StandardMaterial>::default(),
            ));
        }
    }
}

#[derive(Debug, Resource)]
struct DebugState {
    engine: bool,
    wireframe: bool,
    forces: bool,
    physics: bool,
}

impl Default for DebugState {
    fn default() -> Self {
        Self {
            engine: true,
            wireframe: false,
            forces: true,
            physics: false,
        }
    }
}

impl DebugState {
    fn toggle_engine(&mut self) {
        self.engine = !self.engine;
    }
    fn toggle_wireframe(&mut self) {
        self.wireframe = !self.wireframe;
    }
    fn toggle_forces(&mut self) {
        self.forces = !self.forces;
    }
    fn toggle_physics(&mut self) {
        self.physics = !self.physics;
    }
}

fn init_debug_ui(mut commands: Commands, debug_state: Res<DebugState>) {
    info!("initializing debug ui");
    if debug_state.engine {
        commands.trigger(ShowEngineDebug);
    }
    #[cfg(not(target_arch = "wasm32"))]
    if debug_state.wireframe {
        commands.trigger(ShowWireframe);
    }
    if debug_state.forces {
        commands.trigger(ShowForceGizmos);
    }
    if debug_state.physics {
        commands.trigger(ShowPhysicsGizmos);
    }
}

fn trigger_debug_ui(
    mut commands: Commands,
    mut debug_state: ResMut<DebugState>,
    key_input: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindingsConfig>,
) {
    if key_input.just_pressed(key_bindings.debug_controls.toggle_1) {
        debug_state.toggle_engine();
        if debug_state.engine {
            commands.trigger(ShowEngineDebug);
        } else {
            commands.trigger(HideEngineDebug);
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    if key_input.just_pressed(key_bindings.debug_controls.toggle_2) {
        debug_state.toggle_wireframe();
        // Wireframe doesn't work on WASM
        if debug_state.wireframe {
            commands.trigger(ShowWireframe);
        } else {
            commands.trigger(HideWireframe);
        }
    }
    if key_input.just_pressed(key_bindings.debug_controls.toggle_3) {
        debug_state.toggle_forces();
        if debug_state.forces {
            commands.trigger(ShowForceGizmos);
        } else {
            commands.trigger(HideForceGizmos);
        }
    }
    if key_input.just_pressed(key_bindings.debug_controls.toggle_4) {
        debug_state.toggle_physics();
        if debug_state.physics {
            commands.trigger(ShowPhysicsGizmos);
        } else {
            commands.trigger(HidePhysicsGizmos);
        }
    }
}

fn show_engine_debug(
    _trigger: Trigger<ShowEngineDebug>,
    mut store: ResMut<DiagnosticsStore>,
    mut overlay_config: ResMut<FpsOverlayConfig>,
) {
    for diag in store.iter_mut() {
        info!("showing diagnostic {}", diag.path());
        diag.is_enabled = true;
    }
    overlay_config.enabled = true;
}

fn hide_engine_debug(
    _trigger: Trigger<HideEngineDebug>,
    mut store: ResMut<DiagnosticsStore>,
    mut overlay_config: ResMut<FpsOverlayConfig>,
) {
    for diag in store.iter_mut() {
        info!("hiding diagnostic {}", diag.path());
        diag.is_enabled = false;
    }
    overlay_config.enabled = false;
}

fn show_wireframe(
    _trigger: Trigger<ShowWireframe>,
    mut wireframe_config: ResMut<bevy::pbr::wireframe::WireframeConfig>,
) {
    info!("showing wireframe");
    wireframe_config.global = true;
}

fn hide_wireframe(
    _trigger: Trigger<HideWireframe>,
    mut wireframe_config: ResMut<bevy::pbr::wireframe::WireframeConfig>,
) {
    info!("hiding wireframe");
    wireframe_config.global = false;
}

fn show_force_gizmos(
    _trigger: Trigger<ShowForceGizmos>,
    mut store: ResMut<GizmoConfigStore>,
) {
    info!("showing force gizmos");
    let (_, force_config) = store.config_mut::<ForceGizmos>();
    *force_config = ForceGizmos::all();
}

fn hide_force_gizmos(
    _trigger: Trigger<HideForceGizmos>,
    mut store: ResMut<GizmoConfigStore>,
) {
    info!("hiding force gizmos");
    let (_, force_config) = store.config_mut::<ForceGizmos>();
    *force_config = ForceGizmos::none();
}

fn show_physics_gizmos(
    _trigger: Trigger<ShowPhysicsGizmos>,
    mut store: ResMut<GizmoConfigStore>,
) {
    info!("showing physics gizmos");
    let (_, physics_config) = store.config_mut::<PhysicsGizmos>();
    *physics_config = PhysicsGizmos::all();
}

fn hide_physics_gizmos(
    _trigger: Trigger<HidePhysicsGizmos>,
    mut store: ResMut<GizmoConfigStore>,
) {
    info!("hiding physics gizmos");
    let (_, physics_config) = store.config_mut::<PhysicsGizmos>();
    *physics_config = PhysicsGizmos::none();
}

const ARROW_SCALE: f32 = 0.1;

pub struct ForceArrowsPlugin;

impl Plugin for ForceArrowsPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<ForceGizmos>();
        app.register_type::<ForceGizmos>();
        app.add_systems(
            PostUpdate,
            force_arrows
                .run_if(|store: Res<GizmoConfigStore>| store.config::<ForceGizmos>().0.enabled),
        );
    }
}

fn force_arrows(query: Query<&dyn Force>, mut gizmos: Gizmos) {
    for forces in query.iter() {
        for force in forces.iter() {
            let start = force.point_of_application();
            let end = start + force.force() * ARROW_SCALE;
            let color = match force.color() {
                Some(c) => c,
                None => RED.into(),
            };
            gizmos.arrow(start, end, color).with_tip_length(0.3);
        }
    }
}

#[derive(Reflect, GizmoConfigGroup)]
pub struct ForceGizmos {
    /// The scale of the force arrows.
    pub arrow_scale: Option<f32>,
    /// The color of the force arrows. If `None`, the arrows will not be rendered.
    pub arrow_color: Option<Color>,
    /// The length of the arrow tips.
    pub tip_length: Option<f32>,
    /// Determines if the forces should be hidden when not active.
    pub enabled: bool,
}

impl Default for ForceGizmos {
    fn default() -> Self {
        Self {
            arrow_scale: Some(0.1),
            arrow_color: Some(RED.into()),
            tip_length: Some(0.3),
            enabled: false,
        }
    }
}

impl ForceGizmos {
    /// Creates a [`ForceGizmos`] configuration with all rendering options enabled.
    pub fn all() -> Self {
        Self {
            arrow_scale: Some(0.1),
            arrow_color: Some(RED.into()),
            tip_length: Some(0.3),
            enabled: true,
        }
    }

    /// Creates a [`ForceGizmos`] configuration with all rendering options disabled.
    pub fn none() -> Self {
        Self {
            arrow_scale: None,
            arrow_color: None,
            tip_length: None,
            enabled: false,
        }
    }
}
