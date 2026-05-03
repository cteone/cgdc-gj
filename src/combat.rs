use bevy::{
    camera::primitives::Aabb,
    color::palettes::tailwind::RED_300,
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};
use cgdc_gj::{ATTACK_ACCELERATION, ATTACK_SIZE, ENEMY_SIZE, HIT_COOLDOWN_SECONDS};

use crate::{
    enemy::Enemy,
    movement::{Acceleration, AccelerationDirection, Health},
    schedule::InGameSet,
};

#[derive(Component, Default)]
pub struct Attack;

#[derive(Component)]
#[require(Attack)]
pub struct SlashAttack;

#[derive(Component)]
pub struct AttackTimer {
    pub timer: Timer,
}

pub struct CombatPlugin;

#[derive(Component)]
pub struct IsHit {
    pub source: Vec2,
    pub cooldown: Timer,
}

impl Default for IsHit {
    fn default() -> Self {
        IsHit {
            source: Vec2::default(),
            cooldown: Timer::from_seconds(2.5, TimerMode::Once),
        }
    }
}

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (handle_attacks, attack_enemies, enemy_hit)
                .chain()
                .in_set(InGameSet::CollisionDetection),
        );
    }
}

// pub struct Lifetime {
//     timer: Timer
// }

// fn player_attack()
//
fn handle_attacks(
    mut commands: Commands,
    mut query: Query<(Entity, &mut AttackTimer), With<Attack>>,
    time: Res<Time>,
) {
    for (entity, mut attack_timer) in &mut query {
        attack_timer.timer.tick(time.delta());

        if attack_timer.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn attack_enemies(
    enemies: Query<(Entity, Option<&IsHit>), With<Enemy>>,
    attacks: Query<Entity, With<Attack>>,
    mut gizmos: Gizmos,
    transform_helper: TransformHelper,
    mut commands: Commands,
) {
    // for attack in &attacks {
    //     let attack_transform = transform_helper.compute_global_transform(attack)
    //
    //     for enemy in &enemies {}
    // }

    let attack_colliders: Vec<Aabb2d> = attacks
        .into_iter()
        .filter_map(|attack| {
            let attack_transform = transform_helper.compute_global_transform(attack).ok()?;

            let attack_hitbox = Aabb2d::new(attack_transform.translation().xy(), ATTACK_SIZE / 2.0);

            gizmos.rect_2d(
                attack_transform.translation().xy(),
                ATTACK_SIZE / 2.0,
                RED_300,
            );

            Some(attack_hitbox)
        })
        .collect();

    let enemy_colliders: Vec<(Entity, Aabb2d, Option<&IsHit>)> = enemies
        .into_iter()
        .filter_map(|(entity, is_hit)| {
            let enemy_transform = transform_helper.compute_global_transform(entity).ok()?;

            let enemy_hitbox = Aabb2d::new(
                enemy_transform.translation().xy(),
                Vec2::splat(ENEMY_SIZE / 2.0),
            );

            gizmos.rect_2d(
                enemy_transform.translation().xy(),
                Vec2::splat(ENEMY_SIZE / 2.0),
                RED_300,
            );

            Some((entity, enemy_hitbox, is_hit))
        })
        .collect();

    for attack_collider in &attack_colliders {
        for enemy_collider in &enemy_colliders {
            if attack_collider.intersects(&enemy_collider.1) && enemy_collider.2.is_none() {
                commands.entity(enemy_collider.0).insert(IsHit {
                    source: attack_collider.center(),
                    cooldown: Timer::from_seconds(HIT_COOLDOWN_SECONDS, TimerMode::Once),
                });
            }
        }
    }
}

fn enemy_hit(
    mut enemies: Query<
        (
            Entity,
            &Transform,
            &mut Acceleration,
            &mut AccelerationDirection,
            &mut IsHit,
            &mut Health,
        ),
        (With<IsHit>, With<Enemy>),
    >,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, transform, mut acceleration, mut acceleration_direction, mut is_hit, mut health) in
        &mut enemies
    {
        //timer just started
        if is_hit.cooldown.elapsed_secs() == 0.0 {
            health.0 -= 1.0;

            if health.0 <= 0.0 {
                //death effect
                commands.entity(entity).despawn();
            }

            let source = is_hit.source;

            let x = transform.translation.x - source.x;
            let y = transform.translation.y - source.y;

            acceleration_direction.0 = Vec2::new(x, y).normalize_or_zero();
            acceleration.0 = ATTACK_ACCELERATION;
        }

        if is_hit.cooldown.is_finished() {
            commands.entity(entity).remove::<IsHit>();
        }

        is_hit.cooldown.tick(time.delta());
    }
}

// #[derive(EntityEvent)]
// struct EnemyHit {
//     entity: Entity
// }
//
// fn on_enemy_hit (
//     event: On<EnemyHit, Enemy>
//     Query: <Acceleration, AccelerationD
// ) {
//     enemy = event.entity;
// }
