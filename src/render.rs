use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;

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

        app.add_systems(Update, (render,));
    }
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
