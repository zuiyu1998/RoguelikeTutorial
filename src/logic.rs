use crate::render::{Position, Renderable};
use crate::GameState;
use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;

#[derive(Component)]
pub struct LeftMover {}

#[derive(Component)]
pub struct Player {}

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            left_movement.run_if(in_state(GameState::Playing)),
        );
        app.add_systems(Update, (user_input,));

        app.add_systems(
            OnEnter(GameState::Playing),
            (spawn_character, spawn_terminal),
        );
    }
}

pub fn user_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<&mut Position, With<Player>>,
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

    for mut position in q_player.iter_mut() {
        position.movement(x, y);
    }
}

pub fn left_movement(mut q_position: Query<(&mut Position,), With<LeftMover>>) {
    for (mut position,) in q_position.iter_mut() {
        position.x -= 1;

        if position.x < 0 {
            position.x = 79;
        }
    }
}

pub fn spawn_character(mut commands: Commands) {
    for i in 0..10 {
        commands.spawn_empty().insert((
            Position { x: i * 7, y: 20 },
            Renderable {
                glyph: '☺',
                fg: Color::RED,
                bg: Color::BLACK,
            },
            LeftMover {},
        ));
    }

    commands.spawn_empty().insert((
        Position { x: 40, y: 25 },
        Renderable {
            glyph: '@',
            fg: Color::YELLOW,
            bg: Color::BLACK,
        },
        Player {},
    ));
}

pub fn spawn_terminal(mut commands: Commands) {
    let terminal = Terminal::new([80, 50]).with_border(Border::single_line());

    commands.spawn((
        // Spawn the terminal bundle from our terminal
        TerminalBundle::from(terminal),
        Name::new("Terminal"),
    ));
}
