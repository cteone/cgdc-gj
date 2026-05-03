use bevy::{
    color::palettes::tailwind::{BLUE_400, GREEN_300},
    prelude::*,
};
use cgdc_gj::{CANVAS_SIZE, ENEMY_SIZE, TILE_SIZE, TREE_VARIATION};
use rand::Rng;

use crate::{
    animate::FlipSprite,
    collision::Collider,
    enemy::{Enemy, EnemyState},
    player::Player,
    tree::Tree,
};

pub struct RoomLayout {
    pub id: usize,
    pub above: Option<usize>,
    pub below: Option<usize>,
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub tiles: [[Tile; 20]; 15],
}

#[derive(Copy, Clone, Default)]
enum Tile {
    #[default]
    Empty,
    Tree,
    Enemy,
}

#[derive(Component)]
pub struct Room;

#[rustfmt::skip]
impl RoomLayout{
    pub fn new(id: usize, above: Option<usize>, below: Option<usize>, left: Option<usize>, right: Option<usize>, tiles: &str) -> Self{

        let mut new_tiles: [[Tile; 20]; 15] = [[Tile::default(); 20]; 15];

        for (i, line) in tiles.lines().skip(1).enumerate() {
            for (j, char) in line.chars().enumerate() {
                let tile = match char {
                    '^' => Tile::Tree,
                    '.' => Tile::Empty,
                    'E' => Tile::Enemy,
                    _ => Tile::Empty
                };

                new_tiles[i][j] = tile;

            }

        }



        RoomLayout {
            id,
            above,
            below,
            left,
            right,
            tiles: new_tiles,
            // tiles: [
            //     [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Empty, Tile::Empty, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
            //     [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Empty, Tile::Empty, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
            //     [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Empty, Tile::Empty, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
            //     [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Empty, Tile::Empty, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
            //     [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Empty, Tile::Empty, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
            //     [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Empty, Tile::Empty, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
            //     [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
            //     [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
            //     [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
            //     [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
            //     [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
            //     [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
            //     [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
            //     [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
            //     [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
            // ],
        }
    }
}

#[derive(Component, Default)]
pub struct RoomId(pub usize);

// #[derive(Bundle)]
// pub struct Room {
//     id: usize,
//
// };

pub struct RoomsPlugin;

impl Plugin for RoomsPlugin {
    fn build(&self, app: &mut App) {
        //         let room_tile = "
        // ^^^^^^^^^E.^^^^^^^^^
        // ^^^^^^^^^..^^^^^^^^^
        // ^^^^^^^^^..^^^^^^^^^
        // ^^^^^^^^^..^^^^^^^^^
        // ^^^^^^^^^..^^^^^^^^^
        // ^^^^^^^^^..^^^^^^^^^
        // ^^^^^^........^^^^^^
        // ^^^^^^........^^^^^^
        // ^^^^^^........^^^^^^
        // ^^^^^^........^^^^^^
        // ^^^^^^^^^^^^^^^^^^^^
        // ^^^^^^^^^^^^^^^^^^^^
        // ^^^^^^^^^^^^^^^^^^^^
        // ^^^^^^^^^^^^^^^^^^^^
        // ^^^^^^^^^^^^^^^^^^^^
        // ";
        let room_0 = "
^^^^^^^^...^^^^^^^^^
^^^^^^^^...^^^^^^^^^
^^^^^^^^...^^^^^^^^^
^^^^^^^^...^^^^^^^^^
^^^^^^^^...^^^^^^^^^
^^^^^^^^...^^^^^^^^^
^^^^^^........^^^^^^
^^^^^^........^^^^^^
^^^^^^........^^^^^^
^^^^^^........^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
";

        let room_1 = "
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^............
^^^^^^^^............
^^^^^^^^...^^^^^^^^^
^^^^^^^^...^^^^^^^^^
^^^^^^^^...^^^^^^^^^
^^^^^^^^...^^^^^^^^^
^^^^^^^^...^^^^^^^^^
^^^^^^^^...^^^^^^^^^
^^^^^^^^...^^^^^^^^^
^^^^^^^^...^^^^^^^^^
";

        let room_2 = "
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
....................
....................
^^^^^^^^...^^^^^^^^^
^^^^^^^^...^^^^^^^^^
^^^^^^^^...^^^^^^^^^
^^^^^^^^...^^^^^^^^^
^^^^^^^^...^^^^^^^^^
^^^^^^^^...^^^^^^^^^
^^^^^^^^.E.^^^^^^^^^
^^^^^^^^...^^^^^^^^^
";

        app.insert_resource(RoomLayouts {
            value: vec![
                RoomLayout::new(0, Some(1), None, None, None, room_0),
                RoomLayout::new(1, None, Some(0), None, Some(2), room_1),
                RoomLayout::new(2, None, None, Some(1), None, room_2),
            ],
        })
        .add_systems(Startup, startup_room)
        .add_observer(on_room_change);
    }
}

#[derive(Resource)]
pub struct RoomLayouts {
    pub value: Vec<RoomLayout>,
}

#[derive(Event)]
pub struct RoomChange {
    pub target_id: usize,
    pub spawn_location: Vec2,
}

pub enum RoomDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Default)]
pub struct SpawnLocation(pub Vec2);

fn startup_room(
    mut commands: Commands,
    room_layouts: Res<RoomLayouts>,
    asset_server: Res<AssetServer>,
) {
    let room_layout = room_layouts.value.first().unwrap();

    let mut entity_commands = commands.spawn((
        Room,
        RoomId(0),
        SpawnLocation(Vec2::new(0.0, 0.0)),
        Transform::from_xyz(0.0, 0.0, 5.0),
        Visibility::default(),
    ));

    //pixel size

    for i in 0..room_layout.tiles.len() {
        let row = &room_layout.tiles[i];
        let mut rng = rand::rng();
        for j in 0..row.len() {
            let tile = &row[j];
            match tile {
                Tile::Enemy => {
                    let translation = grid_index_to_translation(i, j);
                    println!("{}", translation);
                    entity_commands.with_child((
                        Enemy,
                        EnemyState::Idle,
                        FlipSprite,
                        Sprite {
                            image: asset_server.load("enemy.png"),
                            custom_size: Some(Vec2::splat(ENEMY_SIZE)),
                            ..default()
                        },
                        Transform::from_xyz(translation.x, translation.y, 0.0),
                    ));
                    // enemy_locations.push(grid_index_to_translation(i, j));
                }
                Tile::Tree => {
                    let translation = grid_index_to_translation(i, j);
                    println!("{}", translation);
                    entity_commands.with_child((
                        Collider {
                            center: translation,
                            size: Vec2::splat(TILE_SIZE),
                        },
                        Visibility::default(),
                        Transform::from_xyz(translation.x, translation.y, 0.0),
                        children![
                            (
                                Tree,
                                Sprite {
                                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                                    image: asset_server.load("tree.png"),
                                    flip_y: rng.random_bool(0.5),
                                    ..default()
                                },
                                Transform::from_xyz(
                                    rng.random_range(-TREE_VARIATION..0.0),
                                    rng.random_range(0.0..TREE_VARIATION),
                                    0.0
                                ),
                            ),
                            (
                                Tree,
                                Sprite {
                                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                                    image: asset_server.load("tree.png"),
                                    flip_y: rng.random_bool(0.5),
                                    ..default()
                                },
                                Transform::from_xyz(
                                    rng.random_range(0.0..TREE_VARIATION),
                                    rng.random_range(0.0..TREE_VARIATION),
                                    0.0
                                ),
                            )
                        ],
                    ));
                }
                Tile::Empty => {}
            }
        }
    }

    // commands.spawn((Room, RoomId(room.id)));
}

fn grid_index_to_translation(i: usize, j: usize) -> Vec2 {
    Vec2::new(
        -CANVAS_SIZE.x / 2.0 + TILE_SIZE * j as f32 + TILE_SIZE / 2.0,
        CANVAS_SIZE.y / 2.0 - TILE_SIZE * i as f32 - TILE_SIZE / 2.0,
    )
}

// impl From<RoomLayout> for Room {
//     fn from(room_layout: RoomLayout) -> Self {
//         (RoomId(room_layout.id))
//     }
// }

// fn create_room_bundle_from_layout(room_layout: RoomLayout) {
//     (Room, RoomId(room_layout.id))
// }

pub fn on_room_change(
    room_change: On<RoomChange>,
    room: Single<(Entity, &SpawnLocation), With<Room>>,
    room_layouts: Res<RoomLayouts>,
    player_transform: Single<&mut Transform, With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let (room_entity, spawn_location) = room.into_inner();

    commands.entity(room_entity).despawn();

    let room_id = room_change.target_id;
    let room_layout = room_layouts
        .value
        .iter()
        .find(|room_layout| room_layout.id == room_id)
        .unwrap(); //assert later

    let mut entity_commands = commands.spawn((
        Room,
        RoomId(room_id),
        SpawnLocation(room_change.spawn_location),
        Transform::from_xyz(0.0, 0.0, 5.0),
        Visibility::default(),
    ));

    for i in 0..room_layout.tiles.len() {
        let row = &room_layout.tiles[i];
        let mut rng = rand::rng();
        for j in 0..row.len() {
            let tile = &row[j];
            match tile {
                Tile::Enemy => {
                    let translation = grid_index_to_translation(i, j);
                    println!("{}", translation);
                    entity_commands.with_child((
                        Enemy,
                        EnemyState::Idle,
                        Sprite {
                            image: asset_server.load("enemy.png"),
                            custom_size: Some(Vec2::splat(ENEMY_SIZE)),
                            ..default()
                        },
                        Transform::from_xyz(translation.x, translation.y, 0.0),
                    ));
                    // enemy_locations.push(grid_index_to_translation(i, j));
                }
                Tile::Tree => {
                    let translation = grid_index_to_translation(i, j);
                    println!("{}", translation);
                    entity_commands.with_child((
                        Collider {
                            center: translation,
                            size: Vec2::splat(TILE_SIZE),
                        },
                        Visibility::default(),
                        Transform::from_xyz(translation.x, translation.y, 0.0),
                        children![
                            (
                                Tree,
                                Sprite {
                                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                                    image: asset_server.load("tree.png"),
                                    ..default()
                                },
                                Transform::from_xyz(
                                    rng.random_range(-TREE_VARIATION..0.0),
                                    rng.random_range(0.0..TREE_VARIATION),
                                    0.0
                                ),
                            ),
                            (
                                Tree,
                                Sprite {
                                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                                    image: asset_server.load("tree.png"),
                                    ..default()
                                },
                                Transform::from_xyz(
                                    rng.random_range(0.0..TREE_VARIATION),
                                    rng.random_range(0.0..TREE_VARIATION),
                                    0.0
                                ),
                            )
                        ],
                    ));
                }
                Tile::Empty => {}
            }
        }
    }

    let mut transform = player_transform.into_inner();

    transform.translation.x = room_change.spawn_location.x;
    transform.translation.y = room_change.spawn_location.y;

    println!("change!");
    //destroy old room
    //
    //create and render new room
}
