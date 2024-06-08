use crate::{
    board::Board,
    consts::{SETTINGS_PATH, TILE_SIZE},
    tile::{Material, Tile, TileType, MATERIALS},
};
use bevy::{prelude::*, time::common_conditions::on_timer, window::PrimaryWindow};
use bevy_egui::{egui, EguiContexts};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs::File, str::FromStr};
use std::{io::Write, time::Duration};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                mouse_controls,
                settings_window,
                save_settings.run_if(on_timer(Duration::from_secs(3))),
            ),
        );
    }
}

#[derive(Resource, Serialize, Deserialize)]
struct Settings {
    tile_type: TileType,
    radius: isize,
}

impl Settings {
    fn save_to_file(&self, path: PathBuf) -> std::io::Result<()> {
        let serialized = serde_json::to_string_pretty(self)?;
        File::create(path)?.write_all(serialized.as_bytes())?;
        Ok(())
    }

    fn load_from_file(path: PathBuf) -> serde_json::Result<Self> {
        let contents = std::fs::read_to_string(path).unwrap_or_else(|err| {
            Settings::save_to_file(
                &Settings {
                    tile_type: TileType::Sand,
                    radius: 0,
                },
                PathBuf::from_str(SETTINGS_PATH).unwrap(),
            );
            return "".to_string();
        });
        serde_json::from_str::<Self>(&contents)
    }
}

fn setup(mut commands: Commands) {
    let mut settings = Settings {
        tile_type: TileType::Sand,
        radius: 0,
    };
    if let Ok(saved_settings) = Settings::load_from_file(PathBuf::from_str(SETTINGS_PATH).unwrap())
    {
        settings = saved_settings
    }
    commands.insert_resource(settings);
}

fn save_settings(settings: Res<Settings>) {
    Settings::save_to_file(&settings, PathBuf::from_str(SETTINGS_PATH).unwrap()).unwrap_or_else(
        |err| {
            println!("Failed to save settings :(");
            println!("{:#?}", err);
        },
    );
}

fn settings_window(mut contexts: EguiContexts, mut settings: ResMut<Settings>) {
    egui::Window::new("Settings").show(contexts.ctx_mut(), |ui| {
        ui.label("Material");
        for mat in MATERIALS.iter() {
            let current_item = mat.tile_type;

            ui.radio_value(&mut settings.tile_type, current_item, mat.name);
        }
        ui.label("\nRadius");
        ui.add(egui::Slider::new(&mut settings.radius, 0..=4))
    });
}

fn mouse_controls(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut board: ResMut<Board>,
    settings: Res<Settings>,
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
                    || mouse.pressed(MouseButton::Right)
                {
                    let _ = board.set_radius(
                        tile_x,
                        tile_y,
                        Tile::from_type(settings.tile_type),
                        settings.radius,
                    );
                } else {
                    board.dirty_tiles.push((tile_x, tile_y));
                }
            }
        }
    }
}
