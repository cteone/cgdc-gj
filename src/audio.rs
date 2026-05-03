use bevy::prelude::*;
use bevy_seedling::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SeedlingPlugin::default())
            .add_systems(Startup, setup);
    }
}

#[derive(Component)]
pub struct AmbientSoft;
#[derive(Component)]
pub struct AmbientHard;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SamplePlayer::new(asset_server.load("ambience_2.wav"))
            .looping()
            .with_volume(Volume::Decibels(-2.0)),
        AmbientHard,
    ));
    commands.spawn((
        SamplePlayer::new(asset_server.load("ambience_soft.wav"))
            .looping()
            .with_volume(Volume::Decibels(-2.0)),
        AmbientSoft,
    ));
}
