use bevy::{ecs::query, prelude::*, transform};
use bevy_seedling::sample::{PlaybackSettings, SamplePlayer};
use cgdc_gj::{CANVAS_SIZE, ENEMY_SIZE, ENEMY_TRANSITION_DURATION, TILE_SIZE, TREE_VARIATION};
use rand::Rng;

use crate::{
    animate::FlipSprite,
    audio::{AmbientHard, AmbientSoft},
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
^^^^^^^^...^^^^^^^^^
^^^^^^^^...^^^^^^^^^
^^^^^^^^...^^^^^^^^^
^^^^^^^^...^^^^^^^^^
^^^^^^^^...^^^^^^^^^
...........^^^^^^^^^
...........^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
";

        let room_3 = "
^^^^^^^^^..^^^^^^^^^
^^^^^^^^^..^^^^^^^^^
^^^..............^^^
^^^..............^^^
^^^..............^^^
^................^^^
^E...............^^^
^^^..............^^^
^^^..............^^^
^^^..............^^^
^^^..............^^^
^^^..............^^^
^^^..............^^^
^^^^^^^^^..^^^^^^^^^
^^^^^^^^^..^^^^^^^^^
";

        let room_4 = "
^^^..^^^^^^^^^^^^^^^
^^^...^^^^^^^^^^^^^^
^^^^.E^......^E^^^^^
^^^^^..........^^^^^
^^^^^^........^^^^^^
^^^^^^^......^^^^^^^
^^^^^^^......^^^^^^^
^^^^^^^......^^^^^^^
^^^^^^^^^..^^^^^^^^^
^^^^^^^^^..^^^^^^^^^
^^^^^^^^^..^^^^^^^^^
^^^^^^^^^..^^^^^^^^^
^^^^^^^^^..^^^^^^^^^
^^^^^^^^^..^^^^^^^^^
^^^^^^^^^..^^^^^^^^^
";

        let room_5 = "
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^......^^^^^^^
^^^^^^.........^^^^^
^^^^^^..^^^^...^^^^^
^^^^^..^^^^^^.......
^^^^^..^^^^^^^......
^^^^^..^^^^^^^^^^^^^
^^^^^..^^^^^^^^^^^^^
^^^^^..^^^^^^^^^^^^^
^^^^..^^^^^^^^^^^^^^
^^^^..^^^^^^^^^^^^^^
^^^..^^^^^^^^^^^^^^^
^^^..^^^^^^^^^^^^^^^
";

        let room_6 = "
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^........E.
^^^^^^^^E...........
^^^^^^^^...^^^^^^^^^
^^^^^^^^..^^^^^^^^^^
...^^^^^..^^^^^^^^^^
....^^^^..^^^^^^^^^^
^^........^^^^^^^^^^
^^^......^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
";

        let room_7 = "
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
.................^^^
.................^^^
^^^^^..^^^^^^^^..^^^
^^^^^..^^^^^^......^
^^^^^^..^^^^^......^
^^^^^^^..^^^^E....E^
^^^^^^^..^^^^^^^^^^^
^^^^^^^^....^^^^^^^^
^^^^^^^^^^..^^^^^^^^
^^^^^^^^^^...^^^^^^^
^^^^^^^^^....^^^^^^^
^^^^^^^^^..^^^^^^^^^
^^^^^^^^^..^^^^^^^^^
";

        let room_8 = "
^^^^^^^^^..^^^^^^^^^
^^^^^^^^^..^^^^^^^^^
^^^^^^^^....^^^^^^^^
^^^E............E^^^
^^^^^^^^....^^^^^^^^
^^^^^^^^....^^^^^^^^
...E........^^^^^^^^
^^^^^^^^........E^^^
^^^^^^^^....^^^^^^^^
^^^^^^^^........E^^^
^^^^^^^^....^^^^^^^^
^^^^^^^^....^^^^^^^^
^^^^^^^^^..^^^^^^^^^
^^^^^^^^^..^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
";

        let room_9 = "
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^.......^
^^^^^^^.............
^^^^^..............^
^^^^^........^^^^^^^
^^^^^...^^^^^^^^^^^^
^^^^^...^^^^^^^^^^^^
^^^^^.......^^^^^^^^
^^^^^^......^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
";
        let room_10 = "
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
";
        let room_11 = "
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
";

        let room_12 = "
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^E^^^^...^^^E^^^^
^^^..............^^^
^^^...............E^
^^^..............^^^
^E...............^^^
^^^..............^^^
^^^..............^^^
^^^..............E^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
";
        let room_13 = "
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^^...^^^^^^^^
^^^^^^^^.....^^^^^^^
^^^^^^^.......^^^^^^
";

        let room_14 = "
^^^^^^^.......^^^^^^
^^^^^^.........^^^^^
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
";

        let room_15 = "
^^^^^^^^^^^^^^^^^^^^
^^^^^^^^^^^^^^^^^^^^
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
";

        let room_16 = "
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
";
        app.insert_resource(RoomLayouts {
            value: vec![
                //above below left right
                RoomLayout::new(0, Some(1), None, None, None, room_0),
                RoomLayout::new(1, None, Some(0), None, Some(2), room_1),
                RoomLayout::new(2, Some(3), None, Some(1), None, room_2),
                RoomLayout::new(3, Some(4), Some(2), None, None, room_3),
                RoomLayout::new(4, Some(5), Some(3), None, None, room_4),
                RoomLayout::new(5, None, None, Some(4), Some(6), room_5),
                RoomLayout::new(6, None, None, Some(5), Some(7), room_6),
                RoomLayout::new(7, None, Some(8), Some(6), None, room_7),
                RoomLayout::new(8, Some(7), None, Some(9), None, room_8),
                RoomLayout::new(9, None, Some(10), None, Some(8), room_9),
                RoomLayout::new(10, Some(9), Some(11), None, None, room_10),
                RoomLayout::new(11, Some(10), Some(12), None, None, room_11),
                RoomLayout::new(12, Some(11), Some(13), None, None, room_12),
                RoomLayout::new(13, Some(12), Some(14), None, None, room_13),
                RoomLayout::new(14, Some(13), Some(16), Some(15), Some(15), room_14),
                RoomLayout::new(15, None, Some(16), Some(15), Some(15), room_15),
                RoomLayout::new(16, Some(16), Some(16), Some(16), Some(16), room_16),
            ],
        })
        .add_systems(Startup, startup_room)
        .add_systems(Update, endgame_trigger)
        .add_observer(on_room_change)
        .insert_resource(FinalRoomCounter(0))
        .insert_resource(Endgame(false));
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

#[derive(Component, Default)]
pub struct SpawnLocation(pub Vec2);

fn startup_room(
    mut commands: Commands,
    room_layouts: Res<RoomLayouts>,
    asset_server: Res<AssetServer>,
) {
    let room_layout = room_layouts.value.first().unwrap();

    //TODO: SWITCH BACK
    //
    //let room_layout = room_layouts.value.first().unwrap();

    let mut entity_commands = commands.spawn((
        Room,
        RoomId(room_layout.id),
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
                    entity_commands.with_child((
                        Enemy,
                        EnemyState::Idle,
                        FlipSprite(true),
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
                                    flip_x: rng.random_bool(0.5),
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
                                    flip_x: rng.random_bool(0.5),
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
//
#[derive(Resource)]
pub struct FinalRoomCounter(pub u32);

#[derive(Resource)]
pub struct Endgame(pub bool);

pub fn on_room_change(
    room_change: On<RoomChange>,
    room: Single<(Entity, &SpawnLocation), With<Room>>,
    room_layouts: Res<RoomLayouts>,
    player_transform: Single<&mut Transform, With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut final_room_counter: ResMut<FinalRoomCounter>,
    ambient_soft: Single<
        &mut PlaybackSettings,
        (With<SamplePlayer>, With<AmbientSoft>, Without<AmbientHard>),
    >,
    ambient_hard: Single<
        &mut PlaybackSettings,
        (With<SamplePlayer>, With<AmbientHard>, Without<AmbientSoft>),
    >,
) {
    let (room_entity, _spawn_location) = room.into_inner();

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
                                    flip_x: rng.random_bool(0.5),
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
                                    flip_x: rng.random_bool(0.5),
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

    if room_id == 14 || room_id == 15 {
        let mut settings = ambient_hard.into_inner();
        if *settings.play {
            settings.pause()
        }
    }

    if room_id == 16 {
        final_room_counter.0 += 1;
    }

    if final_room_counter.0 == 2 {
        let mut settings = ambient_soft.into_inner();
        if *settings.play {
            settings.pause()
        }
    }
}

fn endgame_trigger(
    player: Single<Entity, With<Player>>,
    final_room_counter: Res<FinalRoomCounter>,
    transform_helper: TransformHelper,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut endgame: ResMut<Endgame>,
) {
    println!("{}", final_room_counter.0);
    println!("{}", endgame.0);
    if final_room_counter.0 >= 2 && !endgame.0 {
        let pos = transform_helper
            .compute_global_transform(player.into_inner())
            .unwrap()
            .translation();

        if pos.x.abs() < 100.0 && pos.y.abs() < 100.0 {
            endgame.0 = true;
            let directions = [
                Vec2::new(1.0, 0.0).normalize_or_zero() * 200.0,
                Vec2::new(1.0, 1.0).normalize_or_zero() * 200.0,
                Vec2::new(1.0, -1.0).normalize_or_zero() * 200.0,
                Vec2::new(0.0, 1.0).normalize_or_zero() * 200.0,
                Vec2::new(0.0, -1.0).normalize_or_zero() * 200.0,
                Vec2::new(-1.0, 0.0).normalize_or_zero() * 200.0,
                Vec2::new(-1.0, 1.0).normalize_or_zero() * 200.0,
                Vec2::new(-1.0, -1.0).normalize_or_zero() * 200.0,
            ];

            for direction in directions {
                commands.spawn((
                    Enemy,
                    EnemyState::IdleToTarget(Timer::from_seconds(
                        ENEMY_TRANSITION_DURATION,
                        TimerMode::Once,
                    )),
                    FlipSprite(true),
                    Sprite {
                        image: asset_server.load("enemy.png"),
                        custom_size: Some(Vec2::splat(ENEMY_SIZE)),
                        ..default()
                    },
                    Transform::from_xyz(direction.x, direction.y, 0.0),
                ));
            }
        }
    }
}
