use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
    });
}
