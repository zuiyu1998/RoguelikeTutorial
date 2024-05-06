use crate::{
    common::{GameLog, RandomNumberGenerator},
    map::{new_map_rooms_and_corridors, MapEntity},
    player::{PlayerEntity, PlayerPosition},
    spawner::{self, spawn_room, ThemeContext},
    AppState,
};
use bevy::prelude::*;
use bracket_pathfinding::prelude::Point;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), (setup_game,));

        app.add_systems(OnExit(AppState::InGame), clear_game);
    }
}

pub fn clear_game(mut commands: Commands, map_entity: Res<MapEntity>) {
    commands.entity(map_entity.0).despawn_recursive();

    commands.remove_resource::<PlayerEntity>();
    commands.remove_resource::<PlayerPosition>();
    commands.remove_resource::<MapEntity>();
    commands.remove_resource::<GameLog>();
}

fn setup_game(
    mut commands: Commands,
    mut theme_context: ThemeContext,
    mut rng: ResMut<RandomNumberGenerator>,
) {
    let map = new_map_rooms_and_corridors();

    let map_entity = map.spawn_tiles(
        &mut commands,
        &theme_context.texture_assets,
        &mut theme_context.layout_assets,
        &theme_context.theme,
    );

    let first = map.rooms[0].center();

    let player = spawner::player(&mut commands, &mut theme_context, first.0, first.1);

    commands.entity(player).set_parent(map_entity);

    commands.insert_resource(PlayerPosition(Point::new(first.0, first.1)));

    commands.insert_resource(PlayerEntity(player));

    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        spawn_room(
            &mut commands,
            &mut theme_context,
            map_entity,
            &mut rng,
            room,
            i,
            4,
            4,
        )
    }

    commands.insert_resource(MapEntity(map_entity));
    commands.insert_resource(map);
    commands.insert_resource(GameLog::default());
}
