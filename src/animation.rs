// use std::time::Duration;
//
// use bevy::prelude::*;
//
// pub struct AnimationPlugin;
//
// impl Plugin for AnimationPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Startup, setup)
//             .add_systems(Update, execute_animations);
//     }
// }
//
// //from bevy docs
//
// fn trigger_animation<S: Component>(mut animation: Single<&mut AnimationConfig, With<S>>) {
//     // We create a new timer when the animation is triggered
//     animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
// }
//
// #[derive(Component)]
// struct AnimationConfig {
//     first_sprite_index: usize,
//     last_sprite_index: usize,
//     fps: u8,
//     frame_timer: Timer,
// }
//
// impl AnimationConfig {
//     fn new(first: usize, last: usize, fps: u8) -> Self {
//         Self {
//             first_sprite_index: first,
//             last_sprite_index: last,
//             fps,
//             frame_timer: Self::timer_from_fps(fps),
//         }
//     }
//
//     fn timer_from_fps(fps: u8) -> Timer {
//         Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
//     }
// }
//
// // This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// // `last_sprite_index` (both defined in `AnimationConfig`).
// fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
//     for (mut config, mut sprite) in &mut query {
//         // We track how long the current sprite has been displayed for
//         config.frame_timer.tick(time.delta());
//
//         // If it has been displayed for the user-defined amount of time (fps)...
//         if config.frame_timer.just_finished()
//             && let Some(atlas) = &mut sprite.texture_atlas
//         {
//             if atlas.index == config.last_sprite_index {
//                 // ...and it IS the last frame, then we move back to the first frame and stop.
//                 atlas.index = config.first_sprite_index;
//             } else {
//                 // ...and it is NOT the last frame, then we move to the next frame...
//                 atlas.index += 1;
//                 // ...and reset the frame timer to start counting all over again
//                 config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
//             }
//         }
//     }
// }
//
// #[derive(Component)]
// struct AttackSprite;
//
// fn setup(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
// ) {
//     let attack_texture = asset_server.load("slash-sheet.png");
//
//     let attack_layout = TextureAtlasLayout::from_grid(UVec2::new(10, 2), 3, 1, None, None);
//
//     let texture_atlas_layout = texture_atlas_layouts.add(attack_layout);
//
//     let attack_animation_config = AnimationConfig::new(1, 3, 10);
// }
//
//
// fn build_attack_sprite()
