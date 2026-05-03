use bevy::prelude::*;

mod schedule;
use schedule::SchedulePlugin;

mod camera;
use camera::CameraPlugin;

mod lighting;
use lighting::LightingPlugin;

mod background;
use background::BackgroundPlugin;

mod input;
use input::InputPlugin;

mod player;
use player::PlayerPlugin;

mod enemy;
use enemy::EnemyPlugin;

mod tree;

mod movement;
use movement::MovementPlugin;

mod combat;
use combat::CombatPlugin;

// mod debug;
// use debug::DebugPlugin;

mod animate;
use animate::AnimatePlugin;

mod rooms;
use rooms::RoomsPlugin;

mod collision;
use collision::CollisionPlugin;

mod audio;
use audio::AudioPlugin;

mod debug;
use debug::DebugPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            SchedulePlugin,
            DebugPlugin,
            AnimatePlugin,
            AudioPlugin,
            CameraPlugin,
            BackgroundPlugin,
            LightingPlugin,
            InputPlugin,
            RoomsPlugin,
            PlayerPlugin,
            EnemyPlugin,
            MovementPlugin,
            CombatPlugin,
            CollisionPlugin,
        ))
        .run()
}
