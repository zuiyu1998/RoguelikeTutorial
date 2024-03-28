use crate::map::Map;
use crate::render::{Position, Renderable};
use crate::GameState;
use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;

#[derive(Component)]
pub struct Player {}

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (user_input,));

        app.add_systems(OnEnter(GameState::Playing), setup_game);
    }
}

pub fn user_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<&mut Position, With<Player>>,
    map: Res<Map>,
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
        position.movement(x, y, &map);
    }
}

pub fn setup_game(mut commands: Commands, mut map: ResMut<Map>) {
    let terminal = Terminal::new([80, 50]).with_border(Border::single_line());

    commands.spawn((
        // Spawn the terminal bundle from our terminal
        TerminalBundle::from(terminal),
        Name::new("Terminal"),
    ));

    let (map_instance, rooms) = Map::new_map_rooms_and_corridors();

    *map = map_instance;

    let first_room_centerr = rooms[0].center();

    commands.spawn_empty().insert((
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
    ));
}
