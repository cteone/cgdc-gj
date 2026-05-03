use bevy::{
    color::palettes::tailwind::RED_300,
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
use cgdc_gj::{ACCELERATION_DAMPENING_PER_TICK, PLAYER_HITBOX};

use crate::{collision::Collider, player::Player, schedule::InGameSet};

pub struct MovementPlugin;

#[derive(Component, Default)]
pub struct Speed(pub f32);

#[derive(Component, Default)]
pub struct Acceleration(pub f32);

#[derive(Component, Default)]
pub struct AccelerationDirection(pub Vec2);

#[derive(Component, Default)]
pub struct LookingTowards(pub Vec2);

#[derive(Component, Default)]
pub struct Health(pub f32);

#[derive(Component, Default)]
pub struct Direction(pub Vec2);

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (dampen_acceleration, move_entities)
                .chain()
                .in_set(InGameSet::EntityUpdates),
        );
    }
}
fn move_entities(
    mut query: Query<(
        Entity,
        &mut Transform,
        &Speed,
        &Direction,
        Option<&Acceleration>,
        Option<&AccelerationDirection>,
    )>,
    colliders: Query<&Collider>,
    time: Res<Time>,
    transform_helper: TransformHelper,
    mut gizmos: Gizmos,
) {
    for (entity, mut transform, speed, direction, acceleration, acceleration_direction) in
        &mut query
    {
        // let entity_transform = transform_helper
        //     .compute_global_transform(entity)
        //     .unwrap()
        //     .translation();
        //
        // let entity_collider = Aabb2d::new(entity_transform.xy(), PLAYER_HITBOX / 2.0);
        //
        // gizmos.rect_2d(entity_transform.xy(), PLAYER_HITBOX, RED_300);

        let default_acceleration = Acceleration::default();
        let default_acceleration_direction = AccelerationDirection::default();

        let acceleration = acceleration.unwrap_or(&default_acceleration);
        let acceleration_direction =
            acceleration_direction.unwrap_or(&default_acceleration_direction);

        let target = (
            transform.translation.x
                + speed.0 * direction.0.x * time.delta_secs()
                + acceleration.0
                    * acceleration_direction.0.x
                    * time.delta_secs()
                    * time.delta_secs(),
            transform.translation.y
                + speed.0 * direction.0.y * time.delta_secs()
                + acceleration.0
                    * acceleration_direction.0.y
                    * time.delta_secs()
                    * time.delta_secs(),
        );

        let should_move: bool = !colliders.into_iter().any(|collider| {
            let collider = Aabb2d::new(collider.center, collider.size / 2.0);
            // collider.intersects(&entity_collider);
            true
        });

        if should_move {
            transform.translation.x = target.0;
            transform.translation.y = target.1;
        }
    }
}

// fn move_entities(
//     mut query: Query<(
//         Entity,
//         &mut Transform,
//         &Speed,
//         &Direction,
//         Option<&Acceleration>,
//         Option<&AccelerationDirection>,
//     )>,
//     colliders: Query<&Collider>,
//     time: Res<Time>,
//     transform_helper: TransformHelper,
//     mut gizmos: Gizmos,
// ) {
//     for (entity, mut transform, speed, direction, acceleration, acceleration_direction) in
//         &mut query
//     {
//         let entity_transform = transform_helper
//             .compute_global_transform(entity)
//             .unwrap()
//             .translation();
//
//         let entity_collider = Aabb2d::new(entity_transform.xy(), PLAYER_HITBOX / 2.0);
//
//         gizmos.rect_2d(entity_transform.xy(), PLAYER_HITBOX, RED_300);
//
//         let default_acceleration = Acceleration::default();
//         let default_acceleration_direction = AccelerationDirection::default();
//
//         let acceleration = acceleration.unwrap_or(&default_acceleration);
//         let acceleration_direction =
//             acceleration_direction.unwrap_or(&default_acceleration_direction);
//
//         let target = (
//             transform.translation.x
//                 + speed.0 * direction.0.x * time.delta_secs()
//                 + acceleration.0
//                     * acceleration_direction.0.x
//                     * time.delta_secs()
//                     * time.delta_secs(),
//             transform.translation.y
//                 + speed.0 * direction.0.y * time.delta_secs()
//                 + acceleration.0
//                     * acceleration_direction.0.y
//                     * time.delta_secs()
//                     * time.delta_secs(),
//         );
//
//         let should_move: bool = !colliders.into_iter().any(|collider| {
//             let collider = Aabb2d::new(collider.center, collider.size / 2.0);
//             collider.intersects(&entity_collider)
//         });
//
//         if should_move {
//             transform.translation.x = target.0;
//             transform.translation.y = target.1;
//         }
//     }
// }

fn dampen_acceleration(mut query: Query<&mut Acceleration>) {
    for mut acceleration in &mut query {
        if acceleration.0 != 0.0 {
            if acceleration.0 < ACCELERATION_DAMPENING_PER_TICK {
                acceleration.0 = 0.0;
            } else {
                acceleration.0 -= ACCELERATION_DAMPENING_PER_TICK;
            }
        }
    }
}
