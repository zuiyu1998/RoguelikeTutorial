use crate::{
    common::Position,
    loading::TextureAssets,
    map::{new_map, Theme},
    player::Player,
    render::create_sprite_sheet_bundle,
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
    theme: Res<Theme>,
) {
    let map: crate::map::Map = new_map();

    map.spawn_tiles(&mut commands, &texture_assets, &mut layout_assets, &theme);

    commands.insert_resource(map);

    let sprite_bundle = create_sprite_sheet_bundle(
        &texture_assets,
        &mut layout_assets,
        theme.player_to_render(),
    );

    commands.spawn((sprite_bundle, Position { x: 1, y: 1 }, Player));
}
