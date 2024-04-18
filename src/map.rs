use bevy::ecs::system::Resource;
use bevy::prelude::*;
use bracket_random::prelude::RandomNumberGenerator;

use crate::common::Position;
use crate::consts::{MAP_Z_INDEX, SPRITE_SIZE};
use crate::loading::TextureAssets;
use crate::render::{create_sprite_sheet_bundle, Glyph};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Theme>();
    }
}

pub trait MapTheme: 'static + Sync + Send {
    fn tile_to_render(&self, tile_type: TileType) -> Glyph;

    fn player_to_render(&self) -> Glyph;
}

#[derive(Resource, Deref)]
pub struct Theme(Box<dyn MapTheme>);

impl Default for Theme {
    fn default() -> Self {
        Theme(Box::new(DefaultTheme))
    }
}

pub struct DefaultTheme;

impl MapTheme for DefaultTheme {
    fn tile_to_render(&self, tile_type: TileType) -> Glyph {
        match tile_type {
            TileType::Floor => Glyph {
                color: Color::rgba(0.529, 0.529, 0.529, 1.0),
                index: 219,
            },
            TileType::Wall => Glyph {
                color: Color::rgba(0.0, 1.0, 0.0, 1.0),
                index: '#' as usize,
            },
        }
    }

    fn player_to_render(&self) -> Glyph {
        Glyph {
            color: Color::YELLOW,
            index: 64,
        }
    }
}

#[derive(Component)]
pub struct MapTile;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Resource)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<TileType>,
}

pub fn new_map() -> Map {
    let mut map = Map::default();

    let mut rng = RandomNumberGenerator::new();

    for _i in 0..map.height * map.width {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = map.xy_idx(x, y);
        if idx != map.xy_idx(40, 25) {
            map.tiles[idx] = TileType::Wall;
        }
    }

    map
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn get_start_position(&self) -> Vec3 {
        let x = self.width as f32 * SPRITE_SIZE[0] as f32 / -2.0;
        let y = self.height as f32 * SPRITE_SIZE[1] as f32 / -2.0;

        Vec3 {
            x,
            y,
            z: MAP_Z_INDEX,
        }
    }

    pub fn new(width: i32, height: i32) -> Map {
        let width_u = width as usize;
        let height_u = height as usize;

        let tiles = vec![TileType::Floor; width_u * height_u];

        let mut map = Map {
            width,
            height,
            tiles,
        };

        // Make the boundaries walls
        for x in 0..width {
            let index = map.xy_idx(x, 0);
            map.tiles[index] = TileType::Wall;

            let index = map.xy_idx(x, height - 1);
            map.tiles[index] = TileType::Wall;
        }
        for y in 0..height {
            let index = map.xy_idx(0, y);
            map.tiles[index] = TileType::Wall;

            let index = map.xy_idx(width - 1, y);

            map.tiles[index] = TileType::Wall;
        }

        map
    }

    pub fn spawn_tiles(
        &self,
        commands: &mut Commands,
        texture_assets: &TextureAssets,
        layout_assets: &mut Assets<TextureAtlasLayout>,
        theme: &Theme,
    ) {
        let map_entity = commands
            .spawn((
                Name::new("Map"),
                TransformBundle {
                    local: Transform {
                        translation: self.get_start_position(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                VisibilityBundle::default(),
            ))
            .id();
        let mut map_commands = commands.entity(map_entity);

        for x in 0..self.width {
            for y in 0..self.height {
                let index = self.xy_idx(x, y);

                let tile = self.tiles[index];

                let bundle = create_sprite_sheet_bundle(
                    texture_assets,
                    layout_assets,
                    theme.tile_to_render(tile),
                );

                map_commands.with_children(|builder| {
                    builder.spawn((bundle, Position { x, y }, MapTile));
                });
            }
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Map::new(80, 50)
    }
}
