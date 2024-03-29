use crate::map::{Map, Viewshed};
use crate::render::{Position, Renderable};
use crate::GameState;
use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;
use bracket_pathfinding::prelude::Point;
use bracket_random::prelude::RandomNumberGenerator;

#[derive(Component, Debug)]
pub struct Monster {}

#[derive(Component)]
pub struct Player {}

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (user_input,));
        app.add_systems(Update, (monster_ai,).run_if(in_state(GameState::Playing)));

        app.add_systems(OnEnter(GameState::Playing), setup_game);
    }
}

pub fn monster_ai(
    mut set: ParamSet<(
        Query<(&mut Position, &mut Viewshed, &Monster, &Name)>,
        Query<&Position, With<Player>>,
    )>,
) {
    let player = set.p1();
    let player_pos = player.single().clone();

    for (_pos, viewshed, _, name) in set.p0().iter_mut() {
        //占位

        if viewshed
            .visible_tiles
            .contains(&Point::new(player_pos.x, player_pos.y))
        {
            info!("{} shouts insults", name);
        }
    }
}

pub fn user_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<(&mut Position, &mut Viewshed), With<Player>>,
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

    for (mut position, mut viewshed) in q_player.iter_mut() {
        position.movement(x, y, &map, &mut viewshed);
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
        Viewshed {
            visible_tiles: vec![],
            range: 8,
            dirty: true,
        },
        Name::new("Player"),
    ));

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
        ));
    });
}
