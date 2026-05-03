use bevy::prelude::*;

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct Tree;
