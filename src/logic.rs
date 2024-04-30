use crate::{
    common::{GameLog, RandomNumberGenerator},
    map::{new_map_rooms_and_corridors, MapEntity},
    player::{PlayerEntity, PlayerPosition},
    spawner::{self, spawn_room, ThemeContext},
    GameState,
};
use bevy::prelude::*;
use bracket_pathfinding::prelude::Point;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum RunTurnState {
    #[default]
    PreRun,
    //等待输入
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}

pub fn change_to_awaiting_input(
    mut next_state: ResMut<NextState<RunTurnState>>,
    game_ns: Res<State<GameState>>,
) {
    if *game_ns == GameState::Playing {
        next_state.set(RunTurnState::AwaitingInput);
    }
}

pub fn change_to_monster_turn(mut next_state: ResMut<NextState<RunTurnState>>) {
    next_state.set(RunTurnState::MonsterTurn);
}

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<RunTurnState>();

        app.add_systems(OnEnter(GameState::Playing), (setup_game,));

        app.add_systems(OnExit(GameState::Playing), clear_game);

        app.add_systems(
            Update,
            (change_to_awaiting_input,).run_if(in_state(RunTurnState::PreRun)),
        );

        app.add_systems(
            Update,
            (change_to_monster_turn,).run_if(in_state(RunTurnState::PlayerTurn)),
        );

        app.add_systems(
            Update,
            (change_to_awaiting_input,).run_if(in_state(RunTurnState::MonsterTurn)),
        );
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
