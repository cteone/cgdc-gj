use bevy::{
    color::palettes::tailwind::{BLUE_400, GREEN_300},
    prelude::*,
};
use cgdc_gj::{CANVAS_SIZE, ENEMY_SIZE, TILE_SIZE, TREE_VARIATION};
use rand::Rng;

use crate::{collision::Collider, enemy::Enemy, tree::Tree};

struct RoomLayout {
    id: usize,
    above: Option<usize>,
    below: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    tiles: [[Tile; 20]; 15],
}

enum Tile {
    Tree,
    Empty,
    Enemy,
}

#[rustfmt::skip]
impl RoomLayout{
    pub fn new(id: usize) -> Self{
        RoomLayout {
            id: id,
            above: None,
            below: None,
            left: None,
            right: None,
            tiles: [
                [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Empty, Tile::Empty, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
                [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Empty, Tile::Empty, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
                [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Empty, Tile::Empty, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
                [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Empty, Tile::Empty, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
                [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Empty, Tile::Empty, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
                [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Empty, Tile::Empty, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
                [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
                [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
                [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
                [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
                [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
                [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
                [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
                [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
                [Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree, Tile::Tree],
            ],
        }
    }
}

#[derive(Component, Default)]
pub struct RoomId(usize);

// #[derive(Bundle)]
// pub struct Room {
//     id: usize,
//
// };

pub struct RoomsPlugin;

impl Plugin for RoomsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RoomLayouts {
            value: vec![RoomLayout::new(0)],
        })
        .add_systems(Startup, startup_room);
    }
}

#[derive(Resource)]
struct RoomLayouts {
    value: Vec<RoomLayout>,
}

#[derive(Event)]
pub struct RoomChange {
    source_id: usize,
    target_id: usize,
}

fn startup_room(
    mut commands: Commands,
    room_layouts: Res<RoomLayouts>,
    asset_server: Res<AssetServer>,
) {
    let room_layout = room_layouts.value.first().unwrap();

    let mut entity_commands = commands.spawn((
        RoomId(0),
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
                        Sprite {
                            custom_size: Some(Vec2::splat(ENEMY_SIZE)),
                            color: Color::Srgba(BLUE_400),
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
                                    rng.random_range(0.0..TREE_VARIATION),
                                    rng.random_range(-TREE_VARIATION..0.0),
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

// pub fn handle_room_change(room_change: On<RoomChange>, rooms: Res<RoomLayouts>) {
//     //destroy old room
//     //
//     //create and render new room
//
//     let room_bundle = (RoomId);
// }
