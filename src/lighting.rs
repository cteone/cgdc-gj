use bevy::prelude::*;
use bevy_light_2d::prelude::*;

pub struct LightingPlugin;

//attached in camera

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Light2dPlugin);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(AmbientLight2d {
        brightness: 0.0,
        ..default()
    });
}
