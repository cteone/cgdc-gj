use bevy::{color::palettes::tailwind::RED_300, prelude::*};
use cgdc_gj::TILE_SIZE;

use crate::{movement::Direction, player::Player};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(FixedUpdate, player_collisions);
    }
}

// fn player_collisions(
//     player: Single<Entity, Direction, With<Player>>,
//     colliders: Query<&Collider>,
//     transform_helper: TransformHelper,
//     mut gizmos: Gizmos,
// ) {
//     let transform = transform_helper
//         .compute_global_transform(player.into_inner())
//         .unwrap();
//
//     let player_position = transform.translation();
//
//     for collider in &colliders {
//         gizmos.rect_2d(collider.center, Vec2::splat(TILE_SIZE), RED_300)
//     }
//
//     // println!("{}", transform.translation());
// }

#[derive(Component)]
pub struct Collider {
    pub center: Vec2,
    pub size: Vec2,
}
