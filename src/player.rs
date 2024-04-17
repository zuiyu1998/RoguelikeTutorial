use bevy::prelude::*;

use crate::common::Position;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (player_input,));
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
) {
    let mut pos = match q_player.get_single_mut() {
        Ok(pos) => pos,
        Err(_) => return,
    };

    let input = get_input(&keyboard_input);

    pos.x += input.x as i32;
    pos.y += input.y as i32;
}
