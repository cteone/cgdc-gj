use bevy::{color::palettes::tailwind::ORANGE_300, prelude::*};
use bevy_light_2d::light::PointLight2d;
use cgdc_gj::{
    ATTACK_DISTANCE, ATTACK_DURATION_SECONDS, ATTACK_SIZE, PLAYER_HITBOX, PLAYER_LIGHT_RADIUS,
    PLAYER_SIZE, PLAYER_SPEED,
};
use leafwing_input_manager::prelude::{ActionState, InputMap};

use crate::{
    animate::FlipSprite,
    combat::{AttackTimer, SlashAttack},
    input::{Action, movement_input_map},
    movement::{CanMove, Direction, Health, Hitbox, LookingTowards, Speed},
    schedule::InGameSet,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn).add_systems(
            FixedUpdate,
            (player_movement_input, player_attack_input).in_set(InGameSet::UserInput),
        );
    }
}

#[derive(Component)]
#[require(
    Speed,
    Direction,
    Health,
    CanMove,
    Hitbox(PLAYER_HITBOX),
    LookingTowards
)]
pub struct Player;

impl Player {
    fn input_map() -> InputMap<Action> {
        movement_input_map()
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Speed(PLAYER_SPEED),
        FlipSprite(false),
        Sprite {
            image: asset_server.load("player.png"),
            custom_size: Some(Vec2::splat(PLAYER_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Player::input_map(),
        children![
            (PointLight2d {
                radius: PLAYER_LIGHT_RADIUS,
                ..default()
            })
        ],
    ));
}

fn player_movement_input(
    mut player: Single<(&mut Direction, &mut FlipSprite, &mut LookingTowards), With<Player>>,
    actions: Single<&ActionState<Action>, With<Player>>,
) {
    let mut player_dir = Vec2::ZERO;

    if actions.pressed(&Action::Up) {
        player_dir.y += 1.0;
    }
    if actions.pressed(&Action::Down) {
        player_dir.y -= 1.0;
    }
    if actions.pressed(&Action::Right) {
        player_dir.x += 1.0;
    }
    if actions.pressed(&Action::Left) {
        player_dir.x -= 1.0;
    }

    let (mut direction, mut flip_sprite, mut looking_towards) = player.into_inner();

    direction.0 = player_dir.normalize_or_zero();

    if direction.0 != Vec2::ZERO {
        looking_towards.0 = direction.0;
        flip_sprite.0 = true;
    } else {
        flip_sprite.0 = false;
    }
}

fn player_attack_input(
    player: Single<(&LookingTowards, &Transform), With<Player>>,
    actions: Single<&ActionState<Action>, With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if actions.just_pressed(&Action::Attack) {
        let (direction, transform) = player.into_inner();

        let offset = (PLAYER_SIZE / 2.0 + ATTACK_DISTANCE) * direction.0;

        commands.spawn((
            SlashAttack,
            AttackTimer {
                timer: Timer::from_seconds(ATTACK_DURATION_SECONDS, TimerMode::Once),
            },
            Sprite {
                custom_size: Some(ATTACK_SIZE),
                image: asset_server.load("slash.png"),
                ..default()
            },
            Transform {
                translation: transform.translation + offset.extend(0.0),
                rotation: Quat::from_rotation_z(direction.0.to_angle()),
                ..default()
            },
        ));
    }
}
