use crate::consts::{GAME_SIZE, VIEWPORT_SIZE};
use crate::map::{BlocksTile, Map, Viewshed};
use crate::render::{GameTerminal, Position, Renderable};
use crate::ui::GameLog;
use crate::GameState;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use bevy_ascii_terminal::{prelude::*, ToWorld};
use bracket_pathfinding::prelude::{a_star_search, DistanceAlg, Point};
use bracket_random::prelude::RandomNumberGenerator;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum RunTurnState {
    #[default]
    PreRun,
    //等待输入
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}

pub fn change_to_awaiting_input(mut next_state: ResMut<NextState<RunTurnState>>) {
    next_state.set(RunTurnState::AwaitingInput);
}

pub fn change_to_monster_turn(mut next_state: ResMut<NextState<RunTurnState>>) {
    next_state.set(RunTurnState::MonsterTurn);
}

#[derive(Resource, Debug, Clone, Reflect, Deref)]
pub struct PlayerEntity(Entity);

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

pub fn delete_the_dead(
    mut commands: Commands,
    q_combat_stats: Query<(&CombatStats, Entity, &Name)>,
    player_entity: Res<PlayerEntity>,
    mut next_state: ResMut<NextState<GameState>>,
    mut next_run_state: ResMut<NextState<RunTurnState>>,
    mut game_log: ResMut<GameLog>,
) {
    for (combat_stats, entity, name) in q_combat_stats.iter() {
        if combat_stats.hp <= 0 {
            if entity == **player_entity {
                next_state.set(GameState::Menu);
                next_run_state.set(RunTurnState::PreRun);
            } else {
                game_log.entries.push(format!("{} is dead", name));
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub fn apply_damage(
    mut commands: Commands,
    mut q_suffer_damage: Query<(&mut CombatStats, &SufferDamage, Entity)>,
) {
    for (mut stats, damage, entity) in q_suffer_damage.iter_mut() {
        stats.hp -= damage.amount.iter().sum::<i32>();

        commands.entity(entity).remove::<SufferDamage>();
    }
}

pub fn melee_combat(
    mut commands: Commands,
    q_wants_to_melee: Query<(&WantsToMelee, &Parent, Entity)>,
    mut q_combat_stats: Query<(&CombatStats, &Name, Option<&mut SufferDamage>)>,
    mut game_log: ResMut<GameLog>,
) {
    let mut damage_map: HashMap<Entity, Vec<i32>> = HashMap::default();

    for (wants_to_melee, parent, entity) in q_wants_to_melee.iter() {
        let (active, active_name, _) = q_combat_stats.get(parent.get()).unwrap();
        if active.hp < 0 {
            continue;
        }

        let (unactive, unactive_name, _) = q_combat_stats.get(wants_to_melee.target).unwrap();
        if unactive.hp < 0 {
            continue;
        }

        let damage = i32::max(0, active.power - unactive.defense);

        if damage == 0 {
            game_log.entries.push(format!(
                "{} is unable to hurt {}",
                active_name, unactive_name
            ));
        } else {
            game_log.entries.push(format!(
                "{} hits {}, for {} hp.",
                active_name, unactive_name, damage
            ));

            if let Some(tmp_damages) = damage_map.get_mut(&wants_to_melee.target) {
                tmp_damages.push(damage)
            } else {
                damage_map.insert(wants_to_melee.target, vec![damage]);
            }
        }

        commands.entity(entity).despawn_recursive();
    }

    for (entity, damages) in damage_map.into_iter() {
        let (_, _, suffer_damage) = q_combat_stats.get_mut(entity).unwrap();

        if let Some(mut suffer_damage) = suffer_damage {
            suffer_damage.amount.extend_from_slice(&damages);
        } else {
            commands
                .entity(entity)
                .insert(SufferDamage { amount: damages });
        }
    }
}

#[derive(Component, Debug)]
pub struct Monster {}

#[derive(Component)]
pub struct Player {}

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<RunTurnState>();

        app.add_systems(
            Update,
            (change_to_awaiting_input, monster_ai).run_if(in_state(RunTurnState::MonsterTurn)),
        );

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
            (user_input,).run_if(in_state(RunTurnState::AwaitingInput)),
        );

        app.add_systems(
            Update,
            (melee_combat, apply_damage, delete_the_dead).run_if(in_state(GameState::Playing)),
        );

        app.register_type::<WantsToMelee>();
        app.register_type::<SufferDamage>();
        app.register_type::<CombatStats>();

        app.add_systems(OnEnter(GameState::Playing), setup_game);
        app.add_systems(OnExit(GameState::Playing), clear_game);
    }
}

pub fn monster_ai(
    mut commands: Commands,
    mut set: ParamSet<(
        Query<(&mut Position, &mut Viewshed, &Monster, &Name, Entity)>,
        Query<&Position, With<Player>>,
    )>,
    mut map: ResMut<Map>,
    player_entity: Res<PlayerEntity>,
) {
    let q_player = set.p1();
    let player_pos = q_player.single().clone();

    for (mut pos, mut viewshed, _, name, entity) in set.p0().iter_mut() {
        //占位

        if viewshed
            .visible_tiles
            .contains(&Point::new(player_pos.x, player_pos.y))
        {
            info!("{} shouts insults", name);

            let distance = DistanceAlg::Pythagoras.distance2d(
                Point::new(pos.x, pos.y),
                Point::new(player_pos.x, player_pos.y),
            );

            if distance < 1.5 {
                let player = *player_entity.clone();

                commands.entity(entity).with_children(|parent| {
                    parent.spawn(WantsToMelee { target: player });
                });

                return;
            }

            let path = a_star_search(
                map.xy_idx(pos.x, pos.y) as i32,
                map.xy_idx(player_pos.x, player_pos.y) as i32,
                &mut *map,
            );
            info!("path success: {}, setp: {}", path.success, path.steps.len());

            if path.success && path.steps.len() > 1 {
                pos.x = path.steps[1] as i32 % (map.width as i32);
                pos.y = path.steps[1] as i32 / (map.width as i32);
                viewshed.dirty = true;
            }
        }
    }
}

fn handle_user_input(
    position: &mut Position,
    delta_x: i32,
    delta_y: i32,
    map: &Map,
    view: &mut Viewshed,
    q_combat_stats: &Query<&mut CombatStats>,
    commands: &mut EntityCommands,
) {
    let next_x = ((map.width - 1) as i32).min(0.max(position.x + delta_x));
    let next_y = ((map.height - 1) as i32).min(0.max(position.y + delta_y));

    let idx = map.xy_idx(next_x, next_y);

    for potential_target in map.tile_content[idx].iter() {
        let target = q_combat_stats.get(*potential_target);
        match target {
            Err(_e) => {
                error!("tile content index error,entity is :{:?}", potential_target);
            }
            Ok(_t) => {
                // Attack it
                info!("From Hell's Heart, I stab thee!");

                let entity = *potential_target;

                commands.with_children(|parent| {
                    parent.spawn(WantsToMelee { target: entity });
                });

                return; // So we don't move after attacking
            }
        }
    }

    if !map.blocked[idx] {
        position.x = next_x;
        position.y = next_y;

        view.dirty = true;
    }
}

pub fn user_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<(&mut Position, &mut Viewshed, Entity), With<Player>>,
    map: Res<Map>,
    q_combat_stats: Query<&mut CombatStats>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<RunTurnState>>,
) {
    let mut x = 0;
    let mut y = 0;

    if keyboard_input.just_pressed(KeyCode::KeyA) {
        x -= 1;
    }

    if keyboard_input.just_pressed(KeyCode::KeyD) {
        x += 1;
    }

    if keyboard_input.just_pressed(KeyCode::KeyW) {
        y += 1;
    }

    if keyboard_input.just_pressed(KeyCode::KeyS) {
        y -= 1;
    }

    for (mut position, mut viewshed, entity) in q_player.iter_mut() {
        let mut entity_commands = commands.entity(entity);

        handle_user_input(
            &mut position,
            x,
            y,
            &map,
            &mut viewshed,
            &q_combat_stats,
            &mut entity_commands,
        );
    }

    if x != 0 || y != 0 {
        next_state.set(RunTurnState::PlayerTurn);
    }
}

pub fn clear_game(
    mut commands: Commands,
    q_terminal: Query<Entity, With<GameTerminal>>,
    q_position: Query<Entity, With<Position>>,
) {
    for entity in q_terminal.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in q_position.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<PlayerEntity>();
}

pub fn setup_game(mut commands: Commands, mut map: ResMut<Map>) {
    let terminal = Terminal::new([GAME_SIZE[0], GAME_SIZE[1]]).with_border(Border::single_line());

    let term_y = VIEWPORT_SIZE[1] as i32 / 2 - GAME_SIZE[1] as i32 / 2;

    let mut terminal_bundle = TerminalBundle::from(terminal);

    terminal_bundle = terminal_bundle.with_position([0, term_y]);

    commands.spawn((
        terminal_bundle,
        Name::new("GameTerminal"),
        GameTerminal,
        ToWorld::default(),
    ));

    let (map_instance, rooms) = Map::new_map_rooms_and_corridors();

    *map = map_instance;

    let first_room_centerr = rooms[0].center();

    let player = commands
        .spawn_empty()
        .insert((
            Position {
                x: first_room_centerr.0,
                y: first_room_centerr.1,
            },
            Renderable {
                glyph: '@',
                fg: Color::YELLOW,
                bg: Color::BLACK,
            },
            Player {},
            Viewshed {
                visible_tiles: vec![],
                range: 8,
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

    commands.insert_resource(PlayerEntity(player));

    let mut rng = RandomNumberGenerator::new();

    rooms.iter().skip(1).enumerate().for_each(|(i, room)| {
        let center = room.center();

        let roll = rng.roll_dice(1, 2);

        let glyph;
        let name;

        match roll {
            1 => {
                glyph = 'g';
                name = "Goblin".to_string();
            }
            _ => {
                glyph = 'o';
                name = "Orc".to_string();
            }
        }

        commands.spawn_empty().insert((
            Position {
                x: center.0,
                y: center.1,
            },
            Renderable {
                glyph,
                fg: Color::RED,
                bg: Color::BLACK,
            },
            Viewshed {
                visible_tiles: vec![],
                range: 8,
                dirty: true,
            },
            Monster {},
            Name::new(format!("{} #{}", name, i)),
            BlocksTile {},
            CombatStats {
                max_hp: 16,
                hp: 16,
                defense: 1,
                power: 3,
            },
        ));
    });
}
