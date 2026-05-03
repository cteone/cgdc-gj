use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

pub struct AnimatePlugin;

impl Plugin for AnimatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            flip_sprites.run_if(on_timer(Duration::from_millis(250))),
        );
    }
}

#[derive(Component)]
pub struct FlipSprite;

fn flip_sprites(mut query: Query<&mut Sprite, With<FlipSprite>>) {
    for mut sprite in &mut query {
        sprite.flip_x = !sprite.flip_x;
    }
}
