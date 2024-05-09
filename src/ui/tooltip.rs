use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContexts};
use bracket_pathfinding::prelude::Point;

use crate::{
    common::{CombatStats, Position, Viewshed},
    consts::SPRITE_SIZE,
    enemy::Enemy,
    loading::MainCamera,
    map::MapInstance,
    player::Player,
    state::AppStateManager,
    GameState,
};

#[derive(Resource, Default)]
pub struct ToolTipEntity(pub Option<Entity>);

fn update_tooltip(
    // need to get window dimensions
    wnds: Query<&Window, With<PrimaryWindow>>,
    // to get the mouse clicks
    buttons: Res<ButtonInput<MouseButton>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    // query to get all the entities with Name component
    q_names: Query<(&Position, Entity), With<Enemy>>,
    // query to get the player field of view
    player_fov_q: Query<&Viewshed, With<Player>>,
    q_map: Query<&GlobalTransform, With<MapInstance>>,
    mut app_state_manager: AppStateManager,
    mut tooltip_entity: ResMut<ToolTipEntity>,
) {
    // if the user left clicks
    if buttons.just_pressed(MouseButton::Left) {
        // get the primary window
        let wnd = wnds.get_single().unwrap();

        // check if the cursor is in the primary window
        if let Some(pos) = wnd.cursor_position() {
            // assuming there is exactly one main camera entity, so this is OK
            let (camera, camera_transform) = q_camera.single();

            let map_wld = q_map.single().translation().truncate();

            // apply the camera transform
            let point_wld = camera.viewport_to_world_2d(camera_transform, pos).unwrap();

            // transform world coordinates to our grid
            let grid_x =
                (point_wld.x - map_wld.x + SPRITE_SIZE[0] as f32 / 2.0) / SPRITE_SIZE[0] as f32;
            let grid_y =
                (point_wld.y - map_wld.y + SPRITE_SIZE[0] as f32 / 2.0) / SPRITE_SIZE[1] as f32;

            let grid_position = Position {
                x: grid_x as i32,
                y: grid_y as i32,
            };

            // now we go through all the entities with name to see which one is the nearest
            // some variables placeholders to save the entity name and its health
            let mut good_click = false;
            let mut tooltip_entity_tmp: Option<Entity> = None;

            // obtain also player fov
            let player_fov = player_fov_q.single();

            q_names
                .iter()
                .filter(|(pos, _)| {
                    **pos == grid_position
                        && player_fov
                            .visible_tiles
                            .contains(&(Point::new(grid_position.x, grid_position.y)))
                })
                .for_each(|(_, entity)| {
                    good_click = true;
                    tooltip_entity_tmp = Some(entity);
                });

            if good_click {
                app_state_manager.start_tootip();

                tooltip_entity.0 = tooltip_entity_tmp;
            }
        }
    }
}

fn show_tooltip(
    tooltip_entity: ResMut<ToolTipEntity>,
    q_enemy: Query<(&CombatStats, &Name)>,
    mut contexts: EguiContexts,
) {
    let entity = tooltip_entity.0.clone().unwrap();

    if let Ok((stats, name)) = q_enemy.get(entity) {
        egui::show_tooltip(contexts.ctx_mut(), egui::Id::new("my_tooltip"), |ui| {
            ui.label(format!("{} {}:{}", name, stats.hp, stats.max_hp));
        });
    }
}

fn change_to_playing(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_state_manager: AppStateManager,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) || keyboard_input.just_pressed(KeyCode::Space) {
        app_state_manager.start_playing();
    }
}

pub struct TooltipsPlugin;

impl Plugin for TooltipsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ToolTipEntity>();

        app.add_systems(
            Update,
            (update_tooltip,).run_if(in_state(GameState::Playing)),
        );

        app.add_systems(
            Update,
            (change_to_playing, show_tooltip, update_tooltip).run_if(in_state(GameState::ToolTip)),
        );
    }
}
