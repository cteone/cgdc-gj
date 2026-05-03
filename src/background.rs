use bevy::{color::palettes::css::BLACK, prelude::*};
use cgdc_gj::CANVAS_SIZE;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        (Sprite {
            custom_size: Some(CANVAS_SIZE),
            color: Color::Srgba(BLACK),
            ..default()
        }),
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));
}
