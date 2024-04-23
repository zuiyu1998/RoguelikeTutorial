use crate::{
    common::{CombatStats, Position, Viewshed},
    consts::{ENEMY_Z_INDEX, PLAYER_Z_INDEX},
    enemy::{Enemy, EnemyType},
    loading::TextureAssets,
    map::{new_map_rooms_and_corridors, BlocksTile, MapEntity},
    player::{Player, PlayerEntity, PlayerPosition},
    render::create_sprite_sheet_bundle,
    theme::Theme,
    GameState,
};
use bevy::prelude::*;
use bracket_pathfinding::prelude::Point;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (setup_game,));

        app.add_systems(OnExit(GameState::Playing), clear_game);
    }
}

pub fn clear_game(mut commands: Commands, map_entity: Res<MapEntity>) {
    commands.entity(map_entity.0).despawn_recursive();

    commands.remove_resource::<PlayerEntity>();
    commands.remove_resource::<PlayerPosition>();
    commands.remove_resource::<MapEntity>();
}

fn setup_game(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    mut layout_assets: ResMut<Assets<TextureAtlasLayout>>,
    theme: Res<Theme>,
) {
    let mut map = new_map_rooms_and_corridors();

    map.populate_blocked();

    let map_entity = map.spawn_tiles(&mut commands, &texture_assets, &mut layout_assets, &theme);

    let mut sprite_bundle = create_sprite_sheet_bundle(
        &texture_assets,
        &mut layout_assets,
        theme.player_to_render(),
    );

    sprite_bundle.transform.translation.z = PLAYER_Z_INDEX;

    let first = map.rooms[0].center();

    let player = commands
        .spawn((
            sprite_bundle,
            Position {
                x: first.0,
                y: first.1,
            },
            Player,
            Viewshed {
                range: 9,
                visible_tiles: vec![],
                dirty: true,
            },
            Name::new("Player"),
            CombatStats {
                max_hp: 30,
                hp: 30,
                defense: 2,
                power: 5,
            },
        ))
        .id();

    commands.entity(player).set_parent(map_entity);

    commands.insert_resource(PlayerPosition(Point::new(first.0, first.1)));

    commands.insert_resource(PlayerEntity(player));

    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let enemy_tile = EnemyType::G;

        let name;

        match enemy_tile {
            EnemyType::G => {
                name = "Goblin".to_string();
            }
        }

        let enemy_pos = room.center();

        let mut sprite_bundle = create_sprite_sheet_bundle(
            &texture_assets,
            &mut layout_assets,
            theme.enemy_to_render(enemy_tile),
        );

        sprite_bundle.transform.translation.z = ENEMY_Z_INDEX;

        let enemy = commands
            .spawn((
                sprite_bundle,
                Position {
                    x: enemy_pos.0,
                    y: enemy_pos.1,
                },
                Enemy,
                Viewshed {
                    range: 9,
                    visible_tiles: vec![],
                    dirty: true,
                },
                Name::new(format!("{} #{}", &name, i)),
                BlocksTile,
                CombatStats {
                    max_hp: 16,
                    hp: 16,
                    defense: 1,
                    power: 3,
                },
            ))
            .id();

        commands.entity(enemy).set_parent(map_entity);
    }

    commands.insert_resource(MapEntity(map_entity));
    commands.insert_resource(map);
}
