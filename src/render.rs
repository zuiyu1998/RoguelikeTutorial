use crate::consts::SPRITE_SIZE;
use crate::core::TextureAssets;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Glyph {
    pub color: Color,
    pub index: usize,
}

pub fn create_sprite_sheet_bundle(
    texture_assets: &TextureAssets,
    layout_assets: &mut Assets<TextureAtlasLayout>,
    glyph: Glyph,
) -> SpriteSheetBundle {
    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(SPRITE_SIZE[0] as f32, SPRITE_SIZE[1] as f32),
        16,
        16,
        None,
        None,
    );

    SpriteSheetBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(SPRITE_SIZE[0] as f32, SPRITE_SIZE[1] as f32)),
            color: glyph.color,
            ..Default::default()
        },
        atlas: TextureAtlas {
            layout: layout_assets.add(layout),
            index: glyph.index,
        },
        texture: texture_assets.terminal.clone(),
        ..Default::default()
    }
}
