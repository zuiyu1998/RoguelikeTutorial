use bevy::{prelude::*, window::PrimaryWindow};
use bracket_pathfinding::prelude::Point;

use crate::{
    common::{CombatStats, Position, Viewshed},
    consts::SPRITE_SIZE,
    loading::MainCamera,
    logic::RunTurnState,
    map::MapInstance,
    player::Player,
    GameState,
};

use super::FontManager;

fn update_tooltip(
    // need to get window dimensions
    wnds: Query<&Window, With<PrimaryWindow>>,
    // to get the mouse clicks
    buttons: Res<ButtonInput<MouseButton>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    // query to get all the entities with Name component
    q_names: Query<(&Position, &Name, &CombatStats)>,
    // // query to get tooltip text and box
    mut text_box_query: ParamSet<(
        Query<(&mut Text, &mut Visibility), With<ToolTipText>>,
        Query<(&mut Style, &mut Visibility), With<ToolTipBox>>,
    )>,
    // query to get the player field of view
    player_fov_q: Query<&Viewshed, With<Player>>,
    q_map: Query<&GlobalTransform, With<MapInstance>>,
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
            let mut s = String::new();
            let mut maxh = 0;
            let mut currenth = 0;
            // obtain also player fov
            let player_fov = player_fov_q.single();

            q_names
                .iter()
                .filter(|(pos, _, _)| {
                    **pos == grid_position
                        && player_fov
                            .visible_tiles
                            .contains(&(Point::new(grid_position.x, grid_position.y)))
                })
                .for_each(|(_, name, combat_stats)| {
                    s = name.as_str().to_string();
                    good_click = true;
                    // if it also has health component

                    maxh = combat_stats.max_hp;
                    currenth = combat_stats.hp;
                });

            // update tooltip text
            for (mut text, mut visible) in text_box_query.p0().iter_mut() {
                if currenth > 0 {
                    text.sections[0].value = format!("{} HP: {} / {}", s, currenth, maxh);
                } else {
                    text.sections[0].value = format!("{}", s);
                }
                *visible = Visibility::Visible;
            }

            // update box position
            for (mut boxnode, mut visible) in text_box_query.p1().iter_mut() {
                if good_click {
                    boxnode.left = Val::Px(pos.x - 100.0);
                    boxnode.top = Val::Px(pos.y - 40.0);
                    *visible = Visibility::Visible;
                } else {
                    *visible = Visibility::Hidden;
                }
            }
        }
    }
}

fn hide_tooltip(
    mut text_box_query: ParamSet<(
        Query<&mut Visibility, With<ToolTipText>>,
        Query<&mut Visibility, With<ToolTipBox>>,
    )>,
) {
    // update tooltip visibility
    for mut visible in text_box_query.p0().iter_mut() {
        *visible = Visibility::Hidden;
    }

    // update box visibility
    for mut visible in text_box_query.p1().iter_mut() {
        *visible = Visibility::Hidden;
    }
}

pub struct TooltipsPlugin;

impl Plugin for TooltipsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (spawn_tooltip_ui,));
        app.add_systems(
            Update,
            (update_tooltip,).run_if(in_state(RunTurnState::AwaitingInput)),
        );

        app.add_systems(OnExit(RunTurnState::AwaitingInput), (hide_tooltip,));
        app.add_systems(OnExit(GameState::Playing), (clear_tooltip_ui,));
    }
}

#[derive(Component)]
struct ToolTipText;

#[derive(Component)]
struct ToolTipBox;

fn clear_tooltip_ui(mut commands: Commands, q_tooltip_box: Query<Entity, With<ToolTipBox>>) {
    for entity in q_tooltip_box.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_tooltip_ui(mut commands: Commands, font_manager: Res<FontManager>) {
    commands
        // root node, just a black rectangle where the text will be
        .spawn((
            NodeBundle {
                // by default we set visible to false so it starts hidden
                visibility: Visibility::Hidden,
                style: Style {
                    width: Val::Px(300.0),
                    height: Val::Px(30.0),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::rgb(0.0, 0.0, 0.0)),
                ..Default::default()
            },
            ToolTipBox,
        ))
        .with_children(|parent| {
            // text
            parent.spawn((
                TextBundle {
                    visibility: Visibility::Hidden,
                    style: Style {
                        height: Val::Px(20. * 1.),
                        margin: UiRect::all(Val::Auto),
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "Goblin. HP: 2 / 2",
                        TextStyle {
                            font: font_manager.font.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..Default::default()
                },
                ToolTipText,
            ));
        });
}