use crate::{enemy::EnemyType, item::ItemType, map::TileType, render::Glyph};
use bevy::prelude::*;

pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Theme>();
    }
}

pub trait MapTheme: 'static + Sync + Send {
    fn tile_to_render(&self, tile_type: TileType) -> Glyph;
    fn item_to_render(&self, tile_type: ItemType) -> Glyph;

    fn revealed_tile_to_render(&self, tile_type: TileType) -> Glyph;

    fn player_to_render(&self) -> Glyph;

    fn enemy_to_render(&self, enemy_type: EnemyType) -> Glyph;
}

#[derive(Resource, Deref)]
pub struct Theme(Box<dyn MapTheme>);

impl Default for Theme {
    fn default() -> Self {
        Theme(Box::new(DefaultTheme))
    }
}

pub struct DefaultTheme;

impl MapTheme for DefaultTheme {
    fn tile_to_render(&self, tile_type: TileType) -> Glyph {
        match tile_type {
            TileType::Floor => Glyph {
                color: Color::rgba(0.529, 0.529, 0.529, 1.0),
                index: 219,
            },
            TileType::Wall => Glyph {
                color: Color::rgba(0.0, 1.0, 0.0, 1.0),
                index: '#' as usize,
            },
        }
    }

    fn item_to_render(&self, item_type: ItemType) -> Glyph {
        match item_type {
            ItemType::HealthPotion => Glyph {
                color: Color::PURPLE,
                index: '¡' as usize,
            },

            ItemType::MagicMissileScroll => Glyph {
                color: Color::PURPLE,
                index: ')' as usize,
            },
        }
    }

    fn revealed_tile_to_render(&self, tile_type: TileType) -> Glyph {
        match tile_type {
            TileType::Floor => Glyph {
                color: Color::rgba(0.529, 0.529, 0.529, 1.0),
                index: 219,
            },
            TileType::Wall => Glyph {
                color: Color::rgba(0.529, 0.529, 0.529, 1.0),
                index: '#' as usize,
            },
        }
    }

    fn player_to_render(&self) -> Glyph {
        Glyph {
            color: Color::YELLOW,
            index: 64,
        }
    }

    fn enemy_to_render(&self, enemy_type: EnemyType) -> Glyph {
        match enemy_type {
            EnemyType::G => Glyph {
                color: Color::RED,
                index: 'G' as usize,
            },
            EnemyType::O => Glyph {
                color: Color::RED,
                index: 'O' as usize,
            },
        }
    }
}
