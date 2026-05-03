use bevy::{camera::ScalingMode, prelude::*};
use bevy_light_2d::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
        Light2d {
            ambient_light: AmbientLight2d {
                brightness: 0.0,
                ..default()
            },
        },
    ));
}
