use crate::render::Renderable;
use bevy::prelude::*;
use bracket_random::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Map>();
    }
}

#[derive(Resource, Default)]
pub struct Map {
    pub width: usize,
    pub heigth: usize,
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width) + x as usize
    }
}

pub fn new_map() -> Map {
    let mut map = Map {
        tiles: vec![TileType::Floor; 80 * 50],
        width: 80,
        heigth: 50,
    };

    for x in 0..80 {
        let start_idx = map.xy_idx(x, 0);
        let end_idx = map.xy_idx(x, 49);

        map.tiles[start_idx] = TileType::Wall;
        map.tiles[end_idx] = TileType::Wall;
    }
    for y in 0..50 {
        let start_idx = map.xy_idx(0, y);
        let end_idx = map.xy_idx(79, y);

        map.tiles[start_idx] = TileType::Wall;
        map.tiles[end_idx] = TileType::Wall;
    }

    let mut rng = RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = map.xy_idx(x, y);
        if idx != map.xy_idx(40, 25) {
            map.tiles[idx] = TileType::Wall;
        }
    }

    map
}

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

impl TileType {
    pub fn get_renderable(&self) -> Renderable {
        match self {
            TileType::Floor => Renderable {
                fg: Color::rgb(0.5, 0.5, 0.5),
                bg: Color::rgb(0., 0., 0.),
                glyph: '.',
            },
            TileType::Wall => Renderable {
                fg: Color::rgb(0.0, 1.0, 0.0),
                bg: Color::rgb(0., 0., 0.),
                glyph: '#',
            },
        }
    }
}
