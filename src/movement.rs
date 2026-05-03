use bevy::{
    color::palettes::tailwind::RED_300,
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
use cgdc_gj::{ACCELERATION_DAMPENING_PER_TICK, CANVAS_SIZE};

use crate::{
    collision::Collider,
    player::Player,
    rooms::{Room, RoomChange, RoomId, RoomLayouts},
    schedule::InGameSet,
};

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

#[derive(Component, Default)]
pub struct Hitbox(pub Vec2);

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                dampen_acceleration,
                check_collision,
                check_room_change,
                move_entities,
            )
                .chain()
                .in_set(InGameSet::EntityUpdates),
        );
    }
}
//before moving
fn check_collision(
    mut query: Query<(
        Entity,
        &Speed,
        &Direction,
        &mut CanMove,
        Option<&Acceleration>,
        Option<&AccelerationDirection>,
        &Hitbox,
    )>,
    colliders: Query<&Collider>,
    transform_helper: TransformHelper,
    mut gizmos: Gizmos,
    time: Res<Time>,
) {
    let default_acceleration = Acceleration::default();
    let default_acceleration_direction = AccelerationDirection::default();

    for (entity, speed, direction, mut can_move, acceleration, acceleration_direction, hitbox) in
        &mut query
    {
        let entity_transform = transform_helper
            .compute_global_transform(entity)
            .unwrap()
            .translation();

        gizmos.rect_2d(entity_transform.xy(), hitbox.0, RED_300);

        let acceleration = acceleration.unwrap_or(&default_acceleration);
        let acceleration_direction =
            acceleration_direction.unwrap_or(&default_acceleration_direction);

        let target = Vec2::new(
            entity_transform.x
                + speed.0 * direction.0.x * time.delta_secs()
                + acceleration.0
                    * acceleration_direction.0.x
                    * time.delta_secs()
                    * time.delta_secs(),
            entity_transform.y
                + speed.0 * direction.0.y * time.delta_secs()
                + acceleration.0
                    * acceleration_direction.0.y
                    * time.delta_secs()
                    * time.delta_secs(),
        );

        let entity_collider = Aabb2d::new(target, hitbox.0 / 2.0);

        let should_move: bool = !colliders.into_iter().any(|collider| {
            let collider = Aabb2d::new(collider.center, collider.size / 2.0);
            collider.intersects(&entity_collider)
        });

        can_move.0 = should_move;
    }
}

fn check_room_change(
    player: Single<
        (
            Entity,
            &mut CanMove,
            &Speed,
            &Direction,
            Option<&Acceleration>,
            Option<&AccelerationDirection>,
        ),
        With<Player>,
    >,
    mut commands: Commands,
    room: Single<&RoomId, With<Room>>,
    rooms: Res<RoomLayouts>,
    transform_helper: TransformHelper,
    time: Res<Time>,
) {
    let (entity, mut can_move, speed, direction, acceleration, acceleration_direction) =
        player.into_inner();

    let player_location = transform_helper
        .compute_global_transform(entity)
        .unwrap()
        .translation();

    let default_acceleration = Acceleration::default();
    let default_acceleration_direction = AccelerationDirection::default();

    let acceleration = acceleration.unwrap_or(&default_acceleration);
    let acceleration_direction = acceleration_direction.unwrap_or(&default_acceleration_direction);

    let target_location = Vec2::new(
        player_location.x
            + speed.0 * direction.0.x * time.delta_secs()
            + acceleration.0 * acceleration_direction.0.x * time.delta_secs() * time.delta_secs(),
        player_location.y
            + speed.0 * direction.0.y * time.delta_secs()
            + acceleration.0 * acceleration_direction.0.y * time.delta_secs() * time.delta_secs(),
    );

    let source_id = room.into_inner().0;
    let source_room = rooms
        .value
        .iter()
        .find(|room_layout| room_layout.id == source_id)
        .unwrap(); //should exist

    if target_location.x > CANVAS_SIZE.x / 2.0 {
        match source_room.right {
            Some(target_id) => commands.trigger(RoomChange {
                spawn_location: Vec2::new(-player_location.x, player_location.y),
                target_id,
            }),
            None => {
                can_move.0 = false;
            }
        }
    } else if target_location.x < -CANVAS_SIZE.x / 2.0 {
        match source_room.left {
            Some(target_id) => commands.trigger(RoomChange {
                spawn_location: Vec2::new(-player_location.x, player_location.y),
                target_id,
            }),
            None => {
                can_move.0 = false;
            }
        }
    } else if target_location.y > CANVAS_SIZE.y / 2.0 {
        match source_room.above {
            Some(target_id) => commands.trigger(RoomChange {
                spawn_location: Vec2::new(player_location.x, -player_location.y),
                target_id,
            }),
            None => {
                can_move.0 = false;
            }
        }
    } else if target_location.y < -CANVAS_SIZE.y / 2.0 {
        match source_room.below {
            Some(target_id) => commands.trigger(RoomChange {
                spawn_location: Vec2::new(player_location.x, -player_location.y),
                target_id,
            }),
            None => {
                can_move.0 = false;
            }
        }
    }
}

#[derive(Component, Default)]
pub struct CanMove(pub bool);

fn move_entities(
    mut query: Query<(
        &mut Transform,
        &Speed,
        &Direction,
        &CanMove,
        Option<&Acceleration>,
        Option<&AccelerationDirection>,
    )>,
    time: Res<Time>,
    // transform_helper: TransformHelper,
) {
    for (mut transform, speed, direction, can_move, acceleration, acceleration_direction) in
        &mut query
    {
        if can_move.0 {
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

            transform.translation.x = target.0;
            transform.translation.y = target.1;
        }
    }
}

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
