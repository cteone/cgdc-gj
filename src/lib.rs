use bevy::prelude::*;

pub const CANVAS_SIZE: Vec2 = Vec2::new(480., 360.);
pub const TILE_SIZE: f32 = 24.0;

pub const PLAYER_SIZE: f32 = 16.0;
pub const PLAYER_HITBOX: Vec2 = Vec2::new(4.0, 6.0);
pub const PLAYER_SPEED: f32 = 40.0;

pub const ENEMY_SIZE: f32 = 24.0;
pub const ENEMY_HITBOX: Vec2 = Vec2::new(4.0, 6.0);
pub const ENEMY_HEALTH: f32 = 3.0;
pub const ENEMY_IDLE_SPEED: f32 = 40.0;
pub const ENEMY_TARGET_SPEED: f32 = 80.0;
pub const ENEMY_EYESIGHT: f32 = 500.0;

pub const LIGHT_RADIUS: f32 = 45.0;

pub const ATTACK_SIZE: Vec2 = Vec2::new(2.0, 10.0);
pub const ATTACK_DISTANCE: f32 = 6.0;
pub const ATTACK_DURATION_SECONDS: f32 = 0.1;
pub const ATTACK_ACCELERATION: f32 = 15000.0;
pub const HIT_COOLDOWN_SECONDS: f32 = 1.0;

pub const ACCELERATION_DAMPENING_PER_TICK: f32 = 200.0;

pub const TREE_VARIATION: f32 = 10.0;

pub const ROOMS: [&str; 1] = ["
^^^^^^^^^..^^^^^^^^^
^^^^^^^^^..^^^^^^^^^
^^^^^^^^^..^^^^^^^^^
^^^^^^^^^..^^^^^^^^^
^^^^^^^^^..^^^^^^^^^
^^^^^^^^^..^^^^^^^^^
^^^^^^........^^^^^^
^^^^^^........^^^^^^
^^^^^^........^^^^^^
^^^^^^........^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
"];
