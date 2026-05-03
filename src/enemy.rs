use std::time::Duration;

use bevy::{color::palettes::tailwind::BLUE_400, prelude::*, time::common_conditions::on_timer};
use cgdc_gj::{CANVAS_SIZE, ENEMY_HEALTH, PLAYER_SIZE};
use rand::Rng;

pub struct EnemyPlugin;

use crate::{
    movement::{Acceleration, AccelerationDirection, Direction, Health, Speed},
    player::Player,
};

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // add_systems(Startup, spawn)
            // .add_systems(
            //     FixedUpdate,
            //     spawn.run_if(on_timer(Duration::from_millis(100000))),
            // )
            .add_systems(FixedUpdate, calculate_enemy_direction);
    }
}

#[derive(Component)]
#[require(
    Speed(10.0),
    Direction,
    Acceleration,
    AccelerationDirection,
    Health(ENEMY_HEALTH),
    EnemyState
)]
pub struct Enemy;

#[derive(Component)]
pub enum EnemyState {
    IDLE,
    TARGET,
}

impl Default for EnemyState {
    fn default() -> Self {
        EnemyState::IDLE
    }
}

fn spawn(mut commands: Commands) {
    let mut rng = rand::rng();
    let rand_x = rng.random_range(-CANVAS_SIZE.x / 2.0..CANVAS_SIZE.x / 2.0);
    let rand_y = rng.random_range(-CANVAS_SIZE.y / 2.0..CANVAS_SIZE.y / 2.0);

    commands.spawn((
        Enemy,
        Sprite {
            custom_size: Some(Vec2::splat(PLAYER_SIZE)),
            color: Color::Srgba(BLUE_400),
            ..default()
        },
        Transform::from_xyz(rand_x, rand_y, 1.0),
    ));
}

fn calculate_enemy_direction(
    mut enemies: Query<(&mut Direction, &Transform, &EnemyState), With<Enemy>>,
    player: Single<&Transform, With<Player>>,
) {
    for (mut direction, transform, enemy_state) in &mut enemies {
        match enemy_state {
            EnemyState::IDLE => {
                let mut rng = rand::rng();
                let rand_x = rng.random::<i16>();
                let rand_y = rng.random::<i16>();

                let variance = Vec2::new(rand_x as f32, rand_y as f32).normalize_or_zero();

                direction.0 = variance;
            }
            EnemyState::TARGET => {
                const RAND_WEIGHT: f32 = 0.4;
                let x = player.translation.x - transform.translation.x;
                let y = player.translation.y - transform.translation.y;

                let mut rng = rand::rng();
                let rand_x = rng.random::<i16>();
                let rand_y = rng.random::<i16>();

                let variance = Vec2::new(rand_x as f32, rand_y as f32).normalize_or_zero();

                direction.0 = Vec2::new(x, y).normalize_or_zero() + variance;
            }
        }
    }
}

fn handle_state(mut enemies: Query<&mut EnemyState, (With<EnemyState>, With<Enemy>)>) {
    for mut enemy_state in &mut enemies {
        match *enemy_state {
            EnemyState::IDLE => {
                //check if enemy is near player
            }
            EnemyState::TARGET => {}
        }
    }
}
