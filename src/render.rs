use crate::map::Map;
use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;

#[derive(Component)]
pub struct GameTerminal;

///地图上的位置
#[derive(Component, Debug, Clone, Copy, Reflect)]
pub struct Position {
    pub x: i32,
    pub y: i32,
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

///渲染所需数据
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
        app.register_type::<Position>();

        app.add_plugins((TerminalPlugin,));

        app.add_systems(Update, (render,));
    }
}

pub fn render(
    q_position_and_renderable: Query<(&Position, &Renderable)>,
    mut q_render_terminal: Query<&mut Terminal, With<GameTerminal>>,
    map: Res<Map>,
) {
    let mut term = match q_render_terminal.get_single_mut() {
        Ok(term) => term,
        Err(_) => return,
    };
    term.clear();

    for x in 0..map.width {
        for y in 0..map.height {
            let idx = map.xy_idx(x as i32, y as i32);

            if map.revealed_tiles[idx] {
                let mut tile: FormattedTile = map.tiles[idx].get_renderable().into();

                if !map.visible_tiles[idx] {
                    tile = tile.fg(Color::GRAY);
                }

                term.put_char([x, y], tile);
            }
        }
    }

    q_position_and_renderable
        .iter()
        .for_each(|(position, renderable)| {
            let idx = map.xy_idx(position.x, position.y);

            if map.visible_tiles[idx] {
                let tile: FormattedTile = renderable.clone().into();

                term.put_char(position.clone(), tile);
            }
        });
}
