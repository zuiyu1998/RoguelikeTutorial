use bevy::{math::vec2, prelude::*, utils::info};
use bevy_ascii_terminal::{prelude::*, ToWorld};

use crate::{
    consts::{UI_SIZE, VIEWPORT_SIZE},
    logic::{CombatStats, Player},
    render::{GameTerminal, Position},
    GameState,
};

#[derive(Resource)]
pub struct GameLog {
    pub entries: Vec<String>,
}

#[derive(Component)]
pub struct UiTerminal;

pub struct InternalUIPlugin;

impl Plugin for InternalUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_ui_terminal);
        app.add_systems(OnExit(GameState::Playing), clear_ui_terminal);
        app.add_systems(Update, (ui_render,).run_if(in_state(GameState::Playing)));
    }
}

pub fn setup_ui_terminal(mut commands: Commands) {
    let terminal = Terminal::new(UI_SIZE).with_border(Border::single_line());
    let term_y = -(VIEWPORT_SIZE[1] as i32 / 2) + UI_SIZE[1] as i32 / 2 - 1;

    let mut terminal_bundle = TerminalBundle::from(terminal);

    terminal_bundle = terminal_bundle.with_position([0, term_y]);

    commands.spawn((terminal_bundle, Name::new("UiTerminal"), UiTerminal));

    commands.insert_resource(GameLog {
        entries: vec!["Welcome to Rusty Roguelike".to_string()],
    });
}

pub fn clear_ui_terminal(mut commands: Commands, q_terminal: Query<Entity, With<UiTerminal>>) {
    for entity in q_terminal.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<GameLog>();
}

pub fn ui_render(
    q_player: Query<&CombatStats, With<Player>>,
    mut q_render_terminal: Query<&mut Terminal, With<UiTerminal>>,
    game_log: Res<GameLog>,
) {
    let mut term = match q_render_terminal.get_single_mut() {
        Ok(term) => term,
        Err(_) => return,
    };
    term.clear();

    let mut y = 0;

    for log in game_log.entries.iter().rev() {
        if y < 5 {
            term.put_string([4, y], log.fg(Color::WHITE));
        }
        y += 1;
    }

    if let Ok(stats) = q_player.get_single() {
        let hp_string = format!(
            "HP: {} / {}",
            stats.hp.to_string(),
            stats.max_hp.to_string()
        );
        let y = term.side_index(Side::Top) as i32;
        let bar_width = term.width() as i32 - 20;
        let bar_x = term.width() as i32 - bar_width - 1;
        let hp_x = bar_x - hp_string.len() as i32 - 1;

        let fg_color = Color::YELLOW;
        term.put_string([hp_x, y], hp_string.as_str().fg(fg_color));

        term.put_color([bar_x, y], Color::RED.bg());

        progress(&mut term, bar_x, y, stats.max_hp, stats.hp);
    }
}

fn progress(terminal: &mut Terminal, start: i32, y: i32, max: i32, cul: i32) {
    for x in start..start + max {
        terminal.put_color([x, y], Color::rgb(127.0, 33.0, 33.0).bg());
    }

    for x in start..start + cul {
        terminal.put_color([x, y], Color::RED.bg());
    }
}
