use crate::{
    board::Board,
    tile::{Tile, TileType},
};
use bevy::prelude::*;
use rand::Rng;

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
            if tile.tile_type == TileType::Fire {
                new_board = fire_spread(&mut new_board, x, y);
                new_board = fire_strength(&mut new_board, x, y);
            }
            if tile.flows {
                new_board = flow(&mut new_board, x, y);
            }
            if tile.piles {
                new_board = pile(&mut new_board, x, y);
            }
            if tile.gravity {
                new_board = gravity(&mut new_board, x, y);
            }
            if tile.tile_type == TileType::Acid {
                new_board = acid(&mut new_board, x, y);
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

    if bottom.gas || (center.solid && !bottom.solid) {
        board.swap(x, y, x, y.saturating_sub(1))
    }

    board.clone()
}

fn pile(board: &mut Board, x: usize, y: usize) -> Board {
    if board.height() <= y {
        return board.clone();
    }

    let bottom = board.tiles[x][y.saturating_sub(1)];
    let bottom_right = board.tiles[x.saturating_add(1)][y.saturating_sub(1)];
    let bottom_left = board.tiles[x.saturating_sub(1)][y.saturating_sub(1)];

    if bottom.gas {
        // do nothing
        return board.clone();
    } else if !bottom_left.solid && !bottom_right.solid {
        if rand::random::<bool>() {
            board.swap(x, y, x.saturating_sub(1), y.saturating_sub(1));
        } else {
            board.swap(x, y, x.saturating_add(1), y.saturating_sub(1));
        }
    } else if bottom_left.gas {
        board.swap(x, y, x.saturating_sub(1), y.saturating_sub(1));
    } else if bottom_right.gas {
        board.swap(x, y, x.saturating_add(1), y.saturating_sub(1));
    }

    board.clone()
}

fn flow(board: &mut Board, x: usize, y: usize) -> Board {
    if board.tiles.first().unwrap().len() <= y {
        return board.clone();
    }

    let mut bottom = match board.get(x, y.saturating_sub(1)) {
        Some(tile) => tile,
        None => return board.clone(),
    };
    let mut right = match board.get(x.saturating_add(1), y) {
        Some(tile) => tile,
        None => return board.clone(),
    };
    let mut left = match board.get(x.saturating_sub(1), y) {
        Some(tile) => tile,
        None => return board.clone(),
    };

    if !bottom.gas {
        if right.gas && left.gas {
            if rand::random::<bool>() {
                board.swap(x, y, x.saturating_sub(1), y);
            } else {
                board.swap(x, y, x.saturating_add(1), y);
            }
        } else if right.gas {
            board.swap(x, y, x.saturating_add(1), y);
        } else if left.gas {
            board.swap(x, y, x.saturating_sub(1), y);
        }
    }

    board.clone()
}

fn fire_spread(board: &mut Board, x: usize, y: usize) -> Board {
    let mut center = match board.get(x, y) {
        Some(tile) => tile,
        None => return board.clone(),
    };

    let mut top = match board.get(x, y.saturating_add(1)) {
        Some(tile) => tile,
        None => return board.clone(),
    };
    let mut bottom = match board.get(x, y.saturating_sub(1)) {
        Some(tile) => tile,
        None => return board.clone(),
    };
    let mut right = match board.get(x.saturating_add(1), y) {
        Some(tile) => tile,
        None => return board.clone(),
    };
    let mut left = match board.get(x.saturating_sub(1), y) {
        Some(tile) => tile,
        None => return board.clone(),
    };

    let mut should_spread_fire = false;

    for tile in [&mut top, &mut bottom, &mut right, &mut left] {
        if tile.gas && one_in(3) {
            *tile = Tile::from_type(TileType::Fire);
            if let Some(strength) = center.strength {
                (tile).strength = Some(strength.saturating_sub(1));
            }
            should_spread_fire = true;
        } else if tile.flammable && one_in(5) {
            *tile = Tile::from_type(TileType::Fire);
            should_spread_fire = true;
        }
        if tile.strength == Some(0) {
            *tile = Tile::from_type(TileType::None);
        }
    }

    if should_spread_fire {
        center = Tile::from_type(TileType::None);
    }

    board.set(x, y, center);
    board.set(x, y.saturating_add(1), top);
    board.set(x, y.saturating_sub(1), bottom);
    board.set(x.saturating_add(1), y, right);
    board.set(x.saturating_sub(1), y, left);

    board.clone()
}

fn fire_strength(board: &mut Board, x: usize, y: usize) -> Board {
    let mut tile = board.get(x, y).unwrap();

    if let Some(ref mut strength) = tile.strength {
        if *strength > 0 {
            *strength -= 1;
        } else {
            tile = Tile::from_type(TileType::None);
        }
    }
    dbg!(tile);

    board.set(x, y, tile).unwrap();

    board.clone()
}

fn acid(board: &mut Board, x: usize, y: usize) -> Board {
    let mut top = board.tiles[x][y.saturating_add(1)];
    let mut bottom = board.tiles[x][y.saturating_sub(1)];
    let mut right = board.tiles[x.saturating_add(1)][y];
    let mut left = board.tiles[x.saturating_sub(1)][y];

    let mut should_die = false;

    for tile in [&mut top, &mut bottom, &mut right, &mut left] {
        if tile.tile_type != TileType::Acid
            && tile.tile_type != TileType::None
            && tile.tile_type != TileType::Wall
        {
            *tile = Tile::from_type(TileType::None);
            should_die = true;
        }
    }

    if should_die {
        board.set(x, y, Tile::from_type(TileType::None)).unwrap();
    }

    board.set(x, y.saturating_add(1), top);
    board.set(x, y.saturating_sub(1), bottom);
    board.set(x.saturating_add(1), y, right);
    board.set(x.saturating_sub(1), y, left);

    board.clone()
}

fn one_in(every: usize) -> bool {
    rand::thread_rng().gen_range(1..every) == 1
}
