use bevy::{ecs::system::EntityCommands, prelude::*};
use bracket_pathfinding::prelude::{a_star_search, DistanceAlg, Point};
use seldom_state::{prelude::StateMachine, trigger::IntoTrigger};

use crate::{
    common::{Follow, Idle, Position, Viewshed, WantsToMelee},
    map::Map,
    player::{PlayerEntity, PlayerPosition},
    GameState,
};

pub fn add_state_machine(commands: &mut EntityCommands, _enemy: EnemyType) {
    commands.insert((
        StateMachine::default()
            .trans::<Idle, _>(look_player, Follow)
            .trans::<Follow, _>(look_player.not(), Idle)
            .set_trans_logging(true),
        Idle,
        EnemyTimer::default(),
    ));
}

fn look_player(
    In(entity): In<Entity>,
    q_enemy: Query<&Viewshed, With<Enemy>>,
    player_position: Res<PlayerPosition>,
) -> bool {
    if let Ok(viewshed) = q_enemy.get(entity) {
        if viewshed
            .visible_tiles
            .contains(&Point::new(player_position.0.x, player_position.0.y))
        {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

fn enemy_ai(
    mut commands: Commands,
    mut q_enemy: Query<
        (&mut Viewshed, &mut Position, &Name, Entity, &mut EnemyTimer),
        (With<Enemy>, With<Follow>),
    >,
    player_position: Res<PlayerPosition>,
    player_entity: Res<PlayerEntity>,
    mut map: ResMut<Map>,
    time: Res<Time>,
) {
    for (mut viewshed, mut position, name, entity, mut timer) in q_enemy.iter_mut() {
        timer.tick(time.delta());

        if !timer.just_finished() {
            continue;
        }

        info!("{} shouts insults", name);

        let distance = DistanceAlg::Pythagoras.distance2d(
            Point::new(position.x, position.y),
            Point::new(player_position.0.x, player_position.0.y),
        );

        if distance < 1.5 {
            let player = player_entity.0.clone();

            commands.entity(entity).with_children(|parent| {
                parent.spawn(WantsToMelee { target: player });
            });

            return;
        }

        let path = a_star_search(
            map.xy_idx(position.x, position.y) as i32,
            map.xy_idx(player_position.0.x, player_position.0.y) as i32,
            &mut *map,
        );
        if path.success && path.steps.len() > 1 {
            position.x = path.steps[1] as i32 % (map.width as i32);
            position.y = path.steps[1] as i32 / (map.width as i32);
            viewshed.dirty = true;
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (enemy_ai,).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Debug, Component, Deref, DerefMut)]
pub struct EnemyTimer(Timer);

impl Default for EnemyTimer {
    fn default() -> Self {
        EnemyTimer(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EnemyType {
    G,
    O,
}

#[derive(Debug, Component)]
pub struct Enemy;
