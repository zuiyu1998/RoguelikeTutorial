use crate::{
    common::Position,
    loading::TextureAssets,
    map::{new_map_rooms_and_corridors, Theme},
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
    let (map, rooms) = new_map_rooms_and_corridors();

    let map_entity = map.spawn_tiles(&mut commands, &texture_assets, &mut layout_assets, &theme);

    commands.insert_resource(map);

    let sprite_bundle = create_sprite_sheet_bundle(
        &texture_assets,
        &mut layout_assets,
        theme.player_to_render(),
    );
    let first = rooms[0].center();

    commands.entity(map_entity).with_children(|builder| {
        builder.spawn((
            sprite_bundle,
            Position {
                x: first.0,
                y: first.1,
            },
            Player,
        ));
    });
}
