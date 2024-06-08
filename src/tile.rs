use bevy::prelude::*;
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Reflect, Debug, Copy, Component)]
pub struct Tile {
    pub tile_type: TileType,
    pub solid: bool,
    pub flows: bool,
    pub gravity: bool,
    pub color: Color,
    pub gas: bool,
    pub piles: bool,
    pub flammable: bool,
    pub strength: Option<u8>,
}

#[derive(Clone, Default, Reflect, Debug, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileType {
    #[default]
    None,
    Sand,
    Wall,
    Rock,
    Water,
    Fire,
    Wood,
    Acid,
    Dirt,
}

#[derive(Debug)]
pub struct Material {
    pub tile_type: TileType,
    pub name: &'static str,
}

pub const MATERIALS: &[Material] = &[
    Material {
        tile_type: TileType::None,
        name: "Clear",
    },
    Material {
        tile_type: TileType::Rock,
        name: "Rock",
    },
    Material {
        tile_type: TileType::Dirt,
        name: "Dirt",
    },
    Material {
        tile_type: TileType::Sand,
        name: "Sand",
    },
    Material {
        tile_type: TileType::Wall,
        name: "Wall",
    },
    Material {
        tile_type: TileType::Water,
        name: "Water",
    },
    Material {
        tile_type: TileType::Wood,
        name: "Wood",
    },
    Material {
        tile_type: TileType::Fire,
        name: "Fire",
    },
    Material {
        tile_type: TileType::Acid,
        name: "Acid",
    },
];

impl Default for Tile {
    fn default() -> Self {
        Self {
            tile_type: TileType::default(),
            color: Color::hex("000000").unwrap(),
            gravity: false,
            solid: true,
            piles: false,
            flows: false,
            gas: false,
            flammable: false,
            strength: None,
        }
    }
}
impl Tile {
    pub fn from_type(tile_type: TileType) -> Tile {
        match tile_type {
            TileType::None => Tile {
                solid: false,
                color: Color::hex("000000").unwrap(),
                gas: true,
                ..Default::default()
            },
            TileType::Sand => Tile {
                tile_type,
                color: random_color(vec!["f6d7b0", "f2d2a9", "eccca2", "e7c496", "e1bf92"]),
                gravity: true,
                piles: true,
                ..Default::default()
            },
            TileType::Wall => Tile {
                tile_type,
                color: Color::hex("303233").unwrap(),
                ..Default::default()
            },
            TileType::Water => Tile {
                tile_type,
                gravity: true,
                solid: false,
                flows: true,
                piles: true,
                color: vary_color(Color::hex("80ade977").unwrap()),
                ..Default::default()
            },
            TileType::Rock => Tile {
                tile_type,
                gravity: true,
                color: vary_color(Color::hex("5a5a5a").unwrap()),
                ..Default::default()
            },
            TileType::Dirt => Tile {
                tile_type,
                gravity: true,
                color: vary_color(Color::hex("76552b").unwrap()),
                ..Default::default()
            },
            TileType::Fire => Tile {
                tile_type,
                solid: false,
                color: vary_color(Color::hex("f7b538").unwrap()),
                strength: Some(5),
                ..Default::default()
            },
            TileType::Wood => Tile {
                tile_type,
                gravity: true,
                flammable: true,
                color: vary_color(Color::hex("8C5F33").unwrap()),
                ..Default::default()
            },
            TileType::Acid => Tile {
                tile_type,
                gravity: true,
                solid: false,
                flows: true,
                piles: true,
                color: vary_color(Color::hex("70ab5d77").unwrap()),
                ..Default::default()
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
    let offset = rng.gen_range(-variation_range..variation_range);

    // Generate random values for each RGB component
    let variation_r: f32 = offset;
    let variation_g: f32 = offset;
    let variation_b: f32 = offset;

    // Apply the random variation to each RGB component
    rgb[0] = (rgb[0] + variation_r).clamp(0.0, 1.0);
    rgb[1] = (rgb[1] + variation_g).clamp(0.0, 1.0);
    rgb[2] = (rgb[2] + variation_b).clamp(0.0, 1.0);

    Color::rgba(rgb[0], rgb[1], rgb[2], rgb[3])
}
