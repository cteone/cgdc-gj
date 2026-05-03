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
use tree::TreePlugin;

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

mod animation;
use animation::AnimationPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            SchedulePlugin,
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

// fn main() {
//     App::new()
//         .add_plugins((DefaultPlugins, Light2dPlugin))
//         .add_systems(Startup, setup)
//         .run();
// }
//
// fn setup(mut commands: Commands) {
//     commands.spawn((Camera2d, Light2d::default()));
//
//     commands.spawn(PointLight2d {
//         intensity: 3.0,
//         radius: 100.0,
//         ..default()
//     });
//
//     commands.spawn(AmbientLight2d {
//         color: Color::BLACK,
//         brightness: 0.1,
//     });
//
//     commands.spawn(Sprite {
//         custom_size: Some(CANVAS_SIZE),
//         color: Color::Srgba(GREEN_300),
//         ..default()
//     });
// }
