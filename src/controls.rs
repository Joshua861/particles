use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    board::Board,
    consts::TILE_SIZE,
    tile::{Tile, TileType},
};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, controls);
    }
}

fn controls(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut board: ResMut<Board>,
) {
    let window = windows.single();
    let (camera, camera_transform) = cameras.single();

    if mouse.pressed(MouseButton::Left) || mouse.pressed(MouseButton::Right) {
        if let Some(cursor_position) = window.cursor_position() {
            if let Some(world_position) = camera
                .viewport_to_world(camera_transform, cursor_position)
                .map(|ray| ray.origin.truncate())
            {
                let world_position = world_position - Vec2::new(TILE_SIZE / 2.0, TILE_SIZE / 2.0);
                let tile_x = (world_position.x / TILE_SIZE).round() as usize;
                let tile_y = (world_position.y / TILE_SIZE).round() as usize;
                let tile = if mouse.pressed(MouseButton::Left) {
                    TileType::Sand
                } else {
                    TileType::None
                };
                println!("World position: {:?}", world_position);
                println!("Tile position: ({}, {})", tile_x, tile_y);
                if board
                    .get(tile_x, tile_y)
                    .unwrap_or(Tile::from_type(TileType::None))
                    .tile_type
                    == TileType::None
                {
                    let _ = board.set(tile_x, tile_y, Tile::from_type(tile));
                } else {
                    board.dirty_tiles.push((tile_x, tile_y));
                }
            }
        }
    }
}
