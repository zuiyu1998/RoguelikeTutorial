use crate::{
    common::Position, loading::TextureAssets, player::Player, render::create_sprite_sheet_bundle,
    GameState,
};
use bevy::prelude::*;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (setup_game,));
    }
}

fn setup_game(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    mut layout_assets: ResMut<Assets<TextureAtlasLayout>>,
) {
    let sprite_bundle = create_sprite_sheet_bundle(&texture_assets, &mut layout_assets, 64);

    commands.spawn((sprite_bundle, Position { x: 1, y: 1 }, Player));
}
