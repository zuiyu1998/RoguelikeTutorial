use bevy::prelude::*;
use bracket_pathfinding::prelude::{a_star_search, DistanceAlg, Point};

use crate::{
    common::{Position, Viewshed, WantsToMelee},
    map::Map,
    player::{PlayerEntity, PlayerPosition},
    GameState,
};

fn enemy_ai(
    mut commands: Commands,
    mut q_enemy: Query<(&mut Viewshed, &mut Position, &Name, Entity), With<Enemy>>,
    player_position: Res<PlayerPosition>,
    player_entity: Res<PlayerEntity>,
    mut map: ResMut<Map>,
) {
    for (mut viewshed, mut position, name, entity) in q_enemy.iter_mut() {
        if viewshed
            .visible_tiles
            .contains(&Point::new(player_position.0.x, player_position.0.y))
        {
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
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (enemy_ai,).run_if(in_state(GameState::Playing)),
        );

        let fix_time = Time::<Fixed>::from_hz(2.0);

        if let Some(mut _fix_time) = app.world.get_resource_mut::<Time<Fixed>>() {
            *_fix_time = fix_time;
        } else {
            app.insert_resource(fix_time);
        }
    }
}

pub enum EnemyType {
    G,
}

#[derive(Debug, Component)]
pub struct Enemy;
