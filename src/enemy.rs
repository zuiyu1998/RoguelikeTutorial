use bevy::prelude::*;

use crate::{
    common::{Position, Viewshed},
    player::PlayerPosition,
    GameState,
};

fn enemy_ai(
    mut q_enemy: Query<(&Viewshed, &mut Position, &Name), With<Enemy>>,
    player_position: Res<PlayerPosition>,
) {
    for (view, _position, name) in q_enemy.iter_mut() {
        if view.visible_tiles.contains(&player_position.0) {
            info!("{} shouts insults", name);
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (enemy_ai,).run_if(in_state(GameState::Playing)));
    }
}

pub enum EnemyType {
    G,
}

#[derive(Debug, Component)]
pub struct Enemy;
