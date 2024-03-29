use crate::logic::Player;
use crate::render::{Position, Renderable};
use bevy::prelude::*;
use bracket_pathfinding::prelude::*;
use bracket_pathfinding::prelude::{field_of_view, Point};
use bracket_random::prelude::*;

///视野
#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

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

    ///在和其他rect重叠的情况下返回true
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
        app.init_resource::<Map>();

        app.add_systems(PreUpdate, (update_view,));
    }
}

pub fn update_view(
    mut q_view: Query<(&Position, &mut Viewshed)>,
    mut map: ResMut<Map>,
    q_player: Query<Entity, With<Player>>,
) {
    for (pos, mut viewshed) in q_view.iter_mut() {
        if viewshed.dirty {
            viewshed.dirty = false;

            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
            viewshed.visible_tiles.retain(|p| {
                p.x >= 0 && p.x < map.width as i32 && p.y >= 0 && p.y < map.height as i32
            });
        }
    }

    for player_entity in q_player.iter() {
        if let Ok((_, viewshed)) = q_view.get(player_entity) {
            for t in map.visible_tiles.iter_mut() {
                *t = false
            }

            for tile in viewshed.visible_tiles.iter() {
                let idx = map.xy_idx(tile.x, tile.y);

                map.revealed_tiles[idx] = true;

                map.visible_tiles[idx] = true;
            }
        }
    }
}

#[derive(Resource)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
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
}

impl Default for Map {
    fn default() -> Self {
        let map = Map {
            tiles: vec![TileType::Wall; 80 * 50],
            width: 80,
            height: 50,
            revealed_tiles: vec![false; 80 * 50],
            visible_tiles: vec![false; 80 * 50],
        };

        map
    }
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width) + x as usize
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in x1.min(x2)..=x1.max(x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width * self.height {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in y1.min(y2)..=y1.max(y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width * self.height {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    pub fn new_map_rooms_and_corridors() -> (Map, Vec<Rect>) {
        let mut map = Map::default();

        let mut rooms: Vec<Rect> = Vec::new();
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, 80 - w - 1) - 1;
            let y = rng.roll_dice(1, 50 - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            //没有覆盖的room才能更新地图
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

        (map, rooms)
    }
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
