use bevy::prelude::*;
use crate::AppState;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(SPLASH_BACKGROUND_COLOR))
           .add_systems(OnEnter(AppState::Splash), setup_splash)
           .add_systems(Update, update_splash.run_if(in_state(AppState::Splash)))
           .add_systems(OnExit(AppState::Splash), cleanup_splash);
    }
}

const SPLASH_BACKGROUND_COLOR: Color = Color::srgb(0.157, 0.157, 0.157);
const SPLASH_DURATION_SECS: f32 = 1.8;

fn setup_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("textures/splash.png");
    commands.spawn(SpriteBundle {
        texture: texture_handle,
        ..default()
    });
}

fn update_splash(
    time: Res<Time>,
    mut timer: Local<Timer>,
    mut state: ResMut<NextState<AppState>>,
) {
    if timer.elapsed_secs() == 0.0 {
        *timer = Timer::from_seconds(SPLASH_DURATION_SECS, TimerMode::Once);
    }

    if timer.tick(time.delta()).just_finished() {
        state.set(AppState::Loading);
    }
}

fn cleanup_splash(mut commands: Commands) {
    commands.remove_resource::<ClearColor>();
}
