use bevy::prelude::*;

use crate::{
    common::{CombatStats, GameLog},
    player::Player,
    AppState,
};

use super::{FontManager, TopUINode};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), (spwan_bottom_hud,));
        app.add_systems(OnExit(AppState::InGame), (clear_bottom_hud,));

        app.add_systems(
            Update,
            (update_hp_text_and_bar, update_game_log).run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Component)]
pub struct LogUI;

#[derive(Component)]
pub struct HPText;

#[derive(Component)]
pub struct HPBar;

fn update_game_log(game_log: Res<GameLog>, mut text_query: Query<&mut Text, With<LogUI>>) {
    if !game_log.is_changed() {
        return;
    }

    let message_len = game_log.entries.len();
    let mut start = 0;
    let end = message_len;
    if message_len > 4 {
        start = end - 4;
    }

    for mut text in text_query.iter_mut() {
        for (i, entry) in game_log.entries.as_slice()[start..end].iter().enumerate() {
            text.sections[i].value = entry.clone();
        }
    }
}

fn update_hp_text_and_bar(
    mut text_query: Query<&mut Text, With<HPText>>,
    mut bar_query: Query<&mut Style, With<HPBar>>,
    q_combat_stats: Query<&CombatStats, (With<Player>, Changed<CombatStats>)>,
) {
    for combat_stats in q_combat_stats.iter() {
        let (current, max) = (combat_stats.hp, combat_stats.max_hp);

        // update HP text
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("HP: {} / {}", current, max);
        }

        // update HP bar
        let bar_fill = (current as f32 / max as f32) * 100.0;
        for mut bar in bar_query.iter_mut() {
            bar.width = Val::Percent(bar_fill);
        }
    }
}

pub fn clear_bottom_hud(mut commands: Commands, q_top_node: Query<Entity, With<TopUINode>>) {
    for entity in q_top_node.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spwan_bottom_hud(mut commands: Commands, font_manager: Res<FontManager>) {
    commands
        // root node, just a black rectangle where the UI will be
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(100.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::rgb(0.0, 0.0, 0.0)),
                ..Default::default()
            },
            TopUINode,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(50.0),
                        height: Val::Percent(100.0),
                        border: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::rgb(0.65, 0.65, 0.65)),
                    ..Default::default()
                })
                // now inner rectangle
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                // align_items: AlignItems::Stretch,
                                ..Default::default()
                            },
                            background_color: BackgroundColor(Color::rgb(0.0, 0.0, 0.0)),
                            ..Default::default()
                        })
                        // text
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle {
                                    style: Style {
                                        margin: UiRect::all(Val::Px(5.0)),
                                        ..Default::default()
                                    },
                                    // Use `Text` directly
                                    text: Text {
                                        // Construct a `Vec` of `TextSection`s
                                        sections: vec![
                                            TextSection {
                                                value: "Log...\n".to_string(),
                                                style: TextStyle {
                                                    font: font_manager.font.clone(),
                                                    font_size: 20.0,
                                                    color: Color::YELLOW,
                                                },
                                            },
                                            TextSection {
                                                value: "Use the arrow keys to move.\n".to_string(),
                                                style: TextStyle {
                                                    font: font_manager.font.clone(),
                                                    font_size: 20.0,
                                                    color: Color::YELLOW,
                                                },
                                            },
                                            TextSection {
                                                value: "Bump into the enemies to attack them.\n"
                                                    .to_string(),
                                                style: TextStyle {
                                                    font: font_manager.font.clone(),
                                                    font_size: 20.0,
                                                    color: Color::YELLOW,
                                                },
                                            },
                                            TextSection {
                                                value: "Find the amulet to win the game.\n"
                                                    .to_string(),
                                                style: TextStyle {
                                                    font: font_manager.font.clone(),
                                                    font_size: 20.0,
                                                    color: Color::YELLOW,
                                                },
                                            },
                                        ],
                                        justify: JustifyText::Left,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                LogUI,
                            ));
                        });
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(50.0),
                        height: Val::Percent(100.0),
                        border: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..Default::default()
                })
                // now inner rectangle
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                ..Default::default()
                            },
                            background_color: Color::rgb(0.0, 0.0, 0.0).into(),
                            ..Default::default()
                        })
                        // top level with HP information
                        // here we will place both the HP text and the HP bar
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(33.0),
                                        flex_direction: FlexDirection::Row,
                                        ..Default::default()
                                    },
                                    background_color: Color::rgb(0.0, 0.0, 0.0).into(),
                                    ..Default::default()
                                })
                                // container where to place the HP text
                                .with_children(|parent| {
                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(35.0),
                                                height: Val::Percent(100.0),
                                                ..Default::default()
                                            },
                                            background_color: Color::rgb(0.0, 0.0, 0.0).into(),
                                            ..Default::default()
                                        })
                                        // the actual HP text
                                        .with_children(|parent| {
                                            parent.spawn((
                                                TextBundle {
                                                    style: Style {
                                                        height: Val::Px(20. * 1.),
                                                        // Set height to font size * number of text lines
                                                        margin: UiRect {
                                                            left: Val::Auto,
                                                            right: Val::Auto,
                                                            bottom: Val::Auto,
                                                            top: Val::Auto,
                                                        },
                                                        ..Default::default()
                                                    },
                                                    text: Text::from_section(
                                                        "HP: 17 / 20".to_string(),
                                                        TextStyle {
                                                            font_size: 20.0,
                                                            font: font_manager.font.clone(),
                                                            color: Color::rgb(0.99, 0.99, 0.99),
                                                        },
                                                    ),
                                                    ..Default::default()
                                                },
                                                HPText,
                                            ));
                                        });
                                    // outside HP bar
                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(63.0),
                                                height: Val::Px(20. * 1.),
                                                border: UiRect::all(Val::Px(5.0)),
                                                margin: UiRect {
                                                    left: Val::Auto,
                                                    right: Val::Auto,
                                                    bottom: Val::Auto,
                                                    top: Val::Auto,
                                                },
                                                ..Default::default()
                                            },
                                            background_color: Color::rgb(0.5, 0.1, 0.1).into(),
                                            ..Default::default()
                                        })
                                        // inside HP bar
                                        .with_children(|parent| {
                                            parent.spawn((
                                                NodeBundle {
                                                    style: Style {
                                                        width: Val::Percent(50.0),
                                                        height: Val::Percent(100.0),
                                                        ..Default::default()
                                                    },
                                                    background_color: Color::rgb(0.99, 0.1, 0.1)
                                                        .into(),
                                                    ..Default::default()
                                                },
                                                HPBar,
                                            ));
                                        });
                                });
                        });
                });
        });
}
