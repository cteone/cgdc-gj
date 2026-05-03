use bevy::prelude::*;
use bevy_seedling::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SeedlingPlugin::default())
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SamplePlayer::new(asset_server.load("ambience.wav")).looping());
}
