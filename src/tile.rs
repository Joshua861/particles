use bevy::prelude::*;
use rand::{seq::SliceRandom, Rng};

#[derive(Clone, Default, Reflect, Debug, Copy, Component)]
pub struct Tile {
    pub tile_type: TileType,
    pub is_solid: bool,
    pub gravity: bool,
    pub color: Color,
}

#[derive(Clone, Default, Reflect, Debug, Copy, PartialEq, Eq)]
pub enum TileType {
    #[default]
    None,
    Sand,
    Wall,
}

impl Tile {
    pub fn default() -> Tile {
        Tile {
            tile_type: TileType::default(),
            color: Color::hex("000000").unwrap(),
            gravity: false,
            is_solid: true,
        }
    }
    pub fn from_type(tile_type: TileType) -> Tile {
        match tile_type {
            TileType::None => Tile {
                is_solid: false,
                color: Color::hex("000000").unwrap(),
                ..self::default()
            },
            TileType::Sand => Tile {
                tile_type,
                color: random_color(vec!["f6d7b0", "f2d2a9", "eccca2", "e7c496", "e1bf92"]),
                gravity: true,
                is_solid: true,
            },
            TileType::Wall => Tile {
                tile_type,
                color: Color::hex("303233").unwrap(),
                is_solid: true,
                gravity: false,
            },
        }
    }
}

fn random_color(colors: Vec<&str>) -> Color {
    let hex = colors.choose(&mut rand::thread_rng()).unwrap();
    Color::hex(hex).unwrap()
}

fn vary_color(color: Color) -> Color {
    let mut rng = rand::thread_rng();
    let mut rgb = color.as_rgba_f32();

    // Define a range for random variation
    let variation_range = 0.1; // Adjust this to control the amount of variation

    // Generate random values for each RGB component
    let variation_r: f32 = rng.gen_range(-variation_range..variation_range);
    let variation_g: f32 = rng.gen_range(-variation_range..variation_range);
    let variation_b: f32 = rng.gen_range(-variation_range..variation_range);

    // Apply the random variation to each RGB component
    rgb[0] = (rgb[0] + variation_r).clamp(0.0, 1.0);
    rgb[1] = (rgb[1] + variation_g).clamp(0.0, 1.0);
    rgb[2] = (rgb[2] + variation_b).clamp(0.0, 1.0);

    Color::rgb(rgb[0], rgb[1], rgb[2])
}
