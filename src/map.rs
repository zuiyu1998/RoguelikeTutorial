use bevy::ecs::system::Resource;
use bevy::prelude::*;
use bevy::utils::smallvec::SmallVec;
use bracket_pathfinding::prelude::{Algorithm2D, BaseMap, DistanceAlg, Point};
use bracket_random::prelude::RandomNumberGenerator;

use crate::common::Position;
use crate::consts::{MAP_Z_INDEX, SPRITE_SIZE};
use crate::enemy::Enemy;
use crate::loading::TextureAssets;
use crate::render::create_sprite_sheet_bundle;
use crate::theme::Theme;
use crate::GameState;

#[derive(Resource, Deref)]
pub struct MapEntity(pub Entity);

#[derive(Component)]
pub struct MapInstance;

#[derive(Debug)]
pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (map_index,).run_if(in_state(GameState::Playing)));
    }
}

pub fn map_index(
    q_position: Query<(&Position, Entity), With<Enemy>>,
    q_blocks: Query<Entity, With<BlocksTile>>,
    mut map: ResMut<Map>,
) {
    map.populate_blocked();
    map.clear_content_index();

    for (pos, entity) in q_position.iter() {
        let idx = map.xy_idx(pos.x, pos.y);

        if let Ok(_) = q_blocks.get(entity) {
            map.blocked[idx] = true;
            map.tile_content[idx].push(entity);
        }
    }
}

#[derive(Component)]
pub struct MapTile;

#[derive(Component)]
pub struct BlocksTile;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Resource, Debug)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
    pub rooms: Vec<Rect>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub tile_content: Vec<Vec<Entity>>,
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        DistanceAlg::Pythagoras.distance2d(p1, p2)
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let x = idx as i32 % (self.width as i32);
        let y = idx as i32 / (self.width as i32);
        let w = self.width as usize;

        // Cardinal directions
        if self.is_exit_valid(x - 1, y) {
            exits.push((idx - 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((idx + 1, 1.0))
        };
        if self.is_exit_valid(x, y - 1) {
            exits.push((idx - w, 1.0))
        };
        if self.is_exit_valid(x, y + 1) {
            exits.push((idx + w, 1.0))
        };

        // Diagonals
        if self.is_exit_valid(x - 1, y - 1) {
            exits.push(((idx - w) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y - 1) {
            exits.push(((idx - w) + 1, 1.45));
        }
        if self.is_exit_valid(x - 1, y + 1) {
            exits.push(((idx + w) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y + 1) {
            exits.push(((idx + w) + 1, 1.45));
        }

        exits
    }
}

pub fn new_map_rooms_and_corridors() -> Map {
    let mut map = Map::default();

    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, map.width - w - 1) - 1;
        let y = rng.roll_dice(1, map.height - h - 1) - 1;
        let new_room = Rect::new(x, y, w, h);
        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false
            }
        }
        if ok {
            map.apply_room_to_map(&new_room);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rng.range(0, 2) == 1 {
                    map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                    map.apply_vertical_tunnel(prev_y, new_y, new_x);
                } else {
                    map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                    map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                }
            }

            rooms.push(new_room);
        }
    }

    map.rooms = rooms;

    map
}

impl Map {
    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    //将地图中block信息记录在blocked中
    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        let idx = self.xy_idx(x, y);
        !self.blocked[idx]
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in x1.min(x2)..=x1.max(x2) {
            let index = self.xy_idx(x, y);

            if index > 0 && index < (self.width * self.height) as usize {
                self.tiles[index] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in y1.min(y2)..=y1.max(y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < (self.width * self.height) as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let index = self.xy_idx(x, y);
                self.tiles[index] = TileType::Floor;
            }
        }
    }

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

        let tiles = vec![TileType::Wall; width_u * height_u];

        let map = Map {
            width,
            height,
            tiles,
            revealed_tiles: vec![false; width_u * height_u],
            rooms: vec![],
            visible_tiles: vec![false; width_u * height_u],
            blocked: vec![false; width_u * height_u],
            tile_content: vec![vec![]; width_u * height_u],
        };

        map
    }

    pub fn spawn_tiles(
        &self,
        commands: &mut Commands,
        texture_assets: &TextureAssets,
        layout_assets: &mut Assets<TextureAtlasLayout>,
        theme: &Theme,
    ) -> Entity {
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
                MapInstance,
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

        map_entity
    }
}

impl Default for Map {
    fn default() -> Self {
        Map::new(80, 50)
    }
}
