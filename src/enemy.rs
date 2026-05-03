use bevy::{
    math::bounding::{Aabb2d, RayCast2d},
    prelude::*,
};
use bevy_light_2d::light::PointLight2d;
use bevy_seedling::sample::SamplePlayer;
use cgdc_gj::{
    ENEMY_EYESIGHT, ENEMY_HEALTH, ENEMY_HITBOX, ENEMY_LIGHT_OFFSET, ENEMY_LIGHT_RADIUS,
    ENEMY_TARGET_SPEED, ENEMY_TRANSITION_DURATION,
};
use rand::Rng;

pub struct EnemyPlugin;

use crate::{
    animate::FlipSprite,
    collision::Collider,
    movement::{Acceleration, AccelerationDirection, CanMove, Direction, Health, Hitbox, Speed},
    player::Player,
};

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(Startup, spawn)
            // .add_systems(
            //     FixedUpdate,
            //     spawn.run_if(on_timer(Duration::from_millis(100000))),
            // )
            .add_systems(
                FixedUpdate,
                (calculate_enemy_direction, handle_state_change, handle_idle),
            );
    }
}

#[derive(Component)]
#[require(
    Speed(10.0),
    Direction,
    Acceleration,
    AccelerationDirection,
    Health(ENEMY_HEALTH),
    EnemyState,
    Hitbox(ENEMY_HITBOX),
    CanMove,
    FlipSprite(true)
)]
pub struct Enemy;

#[derive(Component)]
pub enum EnemyState {
    Idle,
    Target,
    IdleToTarget(Timer),
}

impl Default for EnemyState {
    fn default() -> Self {
        EnemyState::Idle
    }
}

// fn spawn(mut commands: Commands) {
//     let mut rng = rand::rng();
//     let rand_x = rng.random_range(-CANVAS_SIZE.x / 2.0..CANVAS_SIZE.x / 2.0);
//     let rand_y = rng.random_range(-CANVAS_SIZE.y / 2.0..CANVAS_SIZE.y / 2.0);
//
//     commands.spawn((
//         Enemy,
//         Sprite {
//             custom_size: Some(Vec2::splat(PLAYER_SIZE)),
//             color: Color::Srgba(BLUE_400),
//             ..default()
//         },
//         Transform::from_xyz(0.0, 0.0, 1.0),
//     ));
// }

fn calculate_enemy_direction(
    mut enemies: Query<(&mut Direction, &Transform, &EnemyState), With<Enemy>>,
    player: Single<&Transform, With<Player>>,
) {
    for (mut direction, transform, enemy_state) in &mut enemies {
        match enemy_state {
            EnemyState::Idle => {
                let mut rng = rand::rng();
                let rand_x = rng.random::<i16>();
                let rand_y = rng.random::<i16>();

                let variance = Vec2::new(rand_x as f32, rand_y as f32).normalize_or_zero();

                direction.0 = variance;
            }
            EnemyState::Target => {
                const RAND_WEIGHT: f32 = 0.4;
                let x = player.translation.x - transform.translation.x;
                let y = player.translation.y - transform.translation.y;

                let mut rng = rand::rng();
                let rand_x = rng.random::<i16>();
                let rand_y = rng.random::<i16>();

                let variance = Vec2::new(rand_x as f32, rand_y as f32).normalize_or_zero();

                direction.0 = Vec2::new(x, y).normalize_or_zero() + variance;
            }
            _ => {}
        }
    }
}

fn handle_state_change(
    mut enemies: Query<(Entity, &mut EnemyState, &mut Speed), With<Enemy>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for (entity, mut state, mut speed) in &mut enemies {
        match *state {
            EnemyState::Idle => {}
            EnemyState::IdleToTarget(ref mut timer) => {
                if timer.elapsed_secs() == 0.0 {
                    speed.0 = 0.0;
                    commands.entity(entity).with_child((
                        PointLight2d {
                            radius: ENEMY_LIGHT_RADIUS,
                            intensity: 10.0,
                            ..default()
                        },
                        Transform::from_xyz(ENEMY_LIGHT_OFFSET.x, ENEMY_LIGHT_OFFSET.y, 1.0),
                    ));
                    commands.entity(entity).with_child(SamplePlayer::new(
                        asset_server.load("enemy_idle_to_target.wav"),
                    ));
                }
                if timer.is_finished() {
                    commands.entity(entity).with_child(
                        SamplePlayer::new(asset_server.load("enemy_target.wav")).looping(),
                    );
                    *state = EnemyState::Target;
                } else {
                    timer.tick(time.delta());
                }
            }
            EnemyState::Target => {
                speed.0 = ENEMY_TARGET_SPEED;
            }
        }
    }
}

fn handle_idle(
    mut enemies: Query<(Entity, &mut EnemyState), (With<EnemyState>, With<Enemy>)>,
    player: Single<(Entity, &Hitbox), With<Player>>,
    colliders: Query<&Collider>,
    transform_helper: TransformHelper,
) {
    let (player_entity, player_hitbox) = player.into_inner();

    let player_translation = transform_helper
        .compute_global_transform(player_entity)
        .unwrap()
        .translation();

    let player_collider = Aabb2d::new(player_translation.xy(), player_hitbox.0 / 2.0);

    for (entity, mut enemy_state) in &mut enemies {
        if let EnemyState::Idle = *enemy_state {
            let enemy_translation = transform_helper
                .compute_global_transform(entity)
                .unwrap()
                .translation();

            let x = player_translation.x - enemy_translation.x;
            let y = player_translation.y - enemy_translation.y;

            let raycast = RayCast2d::new(
                enemy_translation.xy(),
                Dir2::from_xy(x, y).unwrap(),
                ENEMY_EYESIGHT,
            );

            let player_dist = raycast.aabb_intersection_at(&player_collider);

            let collider_dist = colliders
                .iter()
                .filter_map(|collider| {
                    let collider = Aabb2d::new(collider.center, collider.size / 2.0);
                    raycast.aabb_intersection_at(&collider)
                })
                .min_by(|a, b| a.partial_cmp(b).unwrap());

            if let Some(player_dist) = player_dist {
                match collider_dist {
                    Some(collider_dist) => {
                        if collider_dist > player_dist {
                            *enemy_state = EnemyState::IdleToTarget(Timer::from_seconds(
                                ENEMY_TRANSITION_DURATION,
                                TimerMode::Once,
                            ));
                        }
                    }
                    None => {
                        *enemy_state = EnemyState::Target;
                    }
                }
            }
        }
    }
}
