use crate::{
    consts::{HEIGHT, TILE_HEIGHT, TILE_SIZE, TILE_WIDTH, WIDTH},
    tile::{Tile, TileType},
};
use bevy::prelude::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_board)
            .add_systems(Update, render_board);
    }
}

#[derive(Resource, Default, Reflect, Debug, Clone)]
pub struct Board {
    pub tiles: Vec<Vec<Tile>>,
    pub dirty_tiles: Vec<(usize, usize)>,
}

impl Board {
    pub fn new(height: usize, width: usize) -> Self {
        let tiles = vec![vec![Tile::from_type(TileType::None); width]; height];

        let mut board = Board {
            tiles,
            dirty_tiles: Vec::new(),
        };

        for y in 0..height {
            for x in 0..width {
                if y == 0 || y == height - 1 || x == 0 || x == width - 1 {
                    let _ = board.set(x, y, Tile::from_type(TileType::Wall));
                }
            }
        }

        board
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Tile> {
        if self.is_in_bounds(x, y) {
            Some(self.tiles[x][y])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, tile: Tile) -> Result<(), String> {
        if self.is_in_bounds(x, y) {
            self.tiles[x][y] = tile;
            self.dirty_tiles.push((x, y)); // Mark tile as dirty
            Ok(())
        } else {
            Err(format!("Index ({}, {}) out of bounds.", x, y).to_string())
        }
    }

    fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.tiles.len() && y < self.tiles.first().unwrap().len()
    }

    fn clear_dirty_tiles(&mut self) {
        self.dirty_tiles.clear();
    }
}

fn init_board(mut commands: Commands) {
    let resource = Board::new(TILE_HEIGHT, TILE_WIDTH);
    commands.insert_resource(resource);
    println!("Board initialized.");
}

// fn render_board(
//     mut commands: Commands,
//     mut board: ResMut<Board>,
//     mut query: Query<Entity, With<Tile>>,
// ) {
//     // Clear existing entities associated with tiles
//     for entity in query.iter_mut() {
//         commands.entity(entity).despawn();
//     }
//
//     // Spawn new entities for tiles that have changed
//     for (x, y) in board
//         .dirty_tiles
//         .drain(..)
//         .collect::<Vec<(usize, usize)>>()
//         .into_iter()
//     {
//         let tile = &board.tiles[x][y];
//         commands
//             .spawn(SpriteBundle {
//                 sprite: Sprite {
//                     color: tile.color,
//                     custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
//                     ..Default::default()
//                 },
//                 transform: Transform::from_xyz(
//                     x as f32 * TILE_SIZE + TILE_SIZE / 2.0,
//                     y as f32 * TILE_SIZE + TILE_SIZE / 2.0,
//                     0.0,
//                 ),
//                 ..Default::default()
//             })
//             .insert(*tile);
//     }
// }

fn render_board(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut tiles: Query<Entity, With<Tile>>,
) {
    for entity in tiles.iter_mut() {
        commands.entity(entity).despawn();
    }

    for (x, row) in board.tiles.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            if tile.tile_type != TileType::None {
                commands
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: tile.color,
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(
                            x as f32 * TILE_SIZE + TILE_SIZE / 2.0,
                            y as f32 * TILE_SIZE + TILE_SIZE / 2.0,
                            0.0,
                        ),
                        ..Default::default()
                    })
                    .insert(*tile);
            }
        }
    }

    board.clear_dirty_tiles();
}
