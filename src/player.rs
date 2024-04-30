use bevy::{prelude::*, utils::petgraph::matrix_graph::Zero};
use bracket_pathfinding::prelude::Point;

use crate::{
    common::{CombatStats, Position, WantsToMelee},
    item::WantsToPickupItem,
    logic::RunTurnState,
    map::Map,
    GameState,
};

#[derive(Resource)]
pub struct PlayerPosition(pub Point);

#[derive(Resource)]
pub struct PlayerEntity(pub Entity);

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (player_input,).run_if(in_state(GameState::Playing)));
    }
}

fn get_input(keyboard_input: &ButtonInput<KeyCode>) -> Vec2 {
    let mut input = Vec2::ZERO;

    if keyboard_input.just_pressed(KeyCode::KeyW) {
        input.y += 1.0;
    }

    if keyboard_input.just_pressed(KeyCode::KeyS) {
        input.y -= 1.0;
    }

    if keyboard_input.just_pressed(KeyCode::KeyD) {
        input.x += 1.0;
    }

    if keyboard_input.just_pressed(KeyCode::KeyA) {
        input.x -= 1.0;
    }

    input
}

pub fn player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<&mut Position, With<Player>>,
    mut player_position: ResMut<PlayerPosition>,
    player_entity: Res<PlayerEntity>,
    map: Res<Map>,
    q_combat_stats: Query<&mut CombatStats>,
    mut commands: Commands,
    mut run_turn_ns: ResMut<NextState<RunTurnState>>,
) {
    let mut pos = match q_player.get_single_mut() {
        Ok(pos) => pos,
        Err(_) => return,
    };

    let input = get_input(&keyboard_input);

    let new_pos_x = pos.x + input.x as i32;
    let new_pos_y = pos.y + input.y as i32;

    let index = map.xy_idx(new_pos_x, new_pos_y);

    for potential_target in map.tile_content[index].iter() {
        let target = q_combat_stats.get(*potential_target);
        match target {
            Err(_e) => {
                error!("tile content index error,entity is :{:?}", potential_target);
            }
            Ok(_t) => {
                // Attack it
                info!("From Hell's Heart, I stab thee!");

                let entity = *potential_target;
                //生成子实体添加攻击组件
                commands.entity(player_entity.0).with_children(|parent| {
                    parent.spawn(WantsToMelee { target: entity });
                });

                return; // So we don't move after attacking
            }
        }
    }

    if map.blocked[index] {
        return;
    }
    if let Some(item_entity) = map.items[index] {
        commands.entity(player_entity.0).with_children(|parent| {
            parent.spawn(WantsToPickupItem {
                collected_by: player_entity.0,
                item: item_entity,
            });
        });
    }

    pos.x = new_pos_x;
    pos.y = new_pos_y;

    player_position.0 = Point::new(new_pos_x, new_pos_y);

    if !input.x.is_zero() {
        run_turn_ns.set(RunTurnState::PlayerTurn);
    }
}
