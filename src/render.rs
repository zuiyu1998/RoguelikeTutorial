use crate::GameState;
use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;

#[derive(Component)]
pub struct LeftMover {}

#[derive(Component)]
pub struct Player {}

#[derive(Component, Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn movement(&mut self, delta_x: i32, delta_y: i32) {
        self.x = 79.min(0.max(self.x + delta_x));
        self.y = 79.min(0.max(self.y + delta_y));
    }
}

impl GridPoint for Position {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }

    fn get_pivot(self) -> Option<Pivot> {
        None
    }
}

#[derive(Component, Debug, Clone)]
pub struct Renderable {
    pub fg: Color,
    pub bg: Color,
    pub glyph: char,
}

impl From<Renderable> for FormattedTile {
    fn from(value: Renderable) -> Self {
        FormattedTile::default()
            .glyph(value.glyph)
            .fg(value.fg)
            .bg(value.bg)
    }
}

pub struct InternalRenderPlugin;

impl Plugin for InternalRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TerminalPlugin,));

        app.add_systems(
            OnEnter(GameState::Playing),
            (spawn_character, spawn_terminal),
        );
        app.add_systems(Update, (render, user_input));
        app.add_systems(
            FixedUpdate,
            left_movement.run_if(in_state(GameState::Playing)),
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

pub fn render(
    q_q_position_and_renderable: Query<(&Position, &Renderable)>,
    mut q_render_terminal: Query<&mut Terminal>,
) {
    let mut term = match q_render_terminal.get_single_mut() {
        Ok(term) => term,
        Err(_) => return,
    };
    term.clear();

    q_q_position_and_renderable
        .iter()
        .for_each(|(position, renderable)| {
            let tile: FormattedTile = renderable.clone().into();

            term.put_char(position.clone(), tile);
        });
}

pub fn spawn_terminal(mut commands: Commands) {
    let terminal = Terminal::new([80, 50]).with_border(Border::single_line());

    commands.spawn((
        // Spawn the terminal bundle from our terminal
        TerminalBundle::from(terminal),
        Name::new("Terminal"),
    ));
}
