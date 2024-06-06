use crate::board::Board;
use crate::tile::Tile;
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, physics);
    }
}

fn physics(mut board: ResMut<Board>) {
    let mut new_board = board.clone();

    for (x, row) in board.tiles.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            if tile.gravity {
                new_board = gravity(&mut new_board, x, y);
            }
        }
    }

    board.tiles = new_board.tiles;
    board.dirty_tiles = new_board.dirty_tiles;
}

fn gravity(board: &mut Board, x: usize, y: usize) -> Board {
    if board.tiles.first().unwrap().len() <= y {
        return board.clone();
    }

    let center = board.tiles[x][y];
    let bottom = board.tiles[x][y.saturating_sub(1)];

    if !bottom.is_solid {
        board.set(x, y, bottom).unwrap();
        board.set(x, y.saturating_sub(1), center).unwrap();
    }

    board.clone()
}
