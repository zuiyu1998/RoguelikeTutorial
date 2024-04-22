use crate::{
    consts::SPRITE_SIZE,
    map::{Map, MapTile},
    player::PlayerEntity,
    theme::Theme,
    GameState,
};
use bevy::prelude::*;
use bracket_pathfinding::prelude::{field_of_view, Point};

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

fn update_visibility(
    mut q_position: Query<(&mut Visibility, &Position, &mut Sprite, Entity)>,
    q_tiles: Query<&MapTile>,
    map: Res<Map>,
    theme: Res<Theme>,
) {
    for (mut visibility, pos, mut sprite, entity) in q_position.iter_mut() {
        let idx = map.xy_idx(pos.x, pos.y);

        if map.visible_tiles[idx] {
            *visibility = Visibility::Visible;

            if q_tiles.get(entity).is_ok() {
                let tile = map.tiles[idx];
                let glyph = theme.tile_to_render(tile);
                sprite.color = glyph.color;
            }
        } else {
            if q_tiles.get(entity).is_ok() && map.revealed_tiles[idx] {
                *visibility = Visibility::Visible;

                let tile = map.tiles[idx];

                let glyph = theme.revealed_tile_to_render(tile);
                sprite.color = glyph.color;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn update_viewshed(
    mut q_viewshed: Query<(&Position, &mut Viewshed, Entity)>,
    mut map: ResMut<Map>,
    player_entity: Res<PlayerEntity>,
) {
    for (pos, mut viewshed, entity) in q_viewshed.iter_mut() {
        if !viewshed.dirty {
            continue;
        }

        viewshed.visible_tiles.clear();
        viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
        viewshed
            .visible_tiles
            .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

        if entity == player_entity.0 {
            for t in map.visible_tiles.iter_mut() {
                *t = false
            }

            for point in viewshed.visible_tiles.iter() {
                let idx = map.xy_idx(point.x, point.y);

                map.revealed_tiles[idx] = true;
                map.visible_tiles[idx] = true;
            }
        }
    }
}

fn keep_position(mut q_position: Query<(&Position, &mut Transform), Or<(Changed<Position>,)>>) {
    for (pos, mut tran) in q_position.iter_mut() {
        let pos = Vec2::new(
            (pos.x * SPRITE_SIZE[0] as i32) as f32,
            (pos.y * SPRITE_SIZE[1] as i32) as f32,
        );

        tran.translation = pos.extend(tran.translation.z);
    }
}

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Position>();

        app.add_systems(
            Update,
            (keep_position, update_viewshed, update_visibility)
                .run_if(in_state(GameState::Playing)),
        );
    }
}
