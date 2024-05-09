use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    common::{CombatStats, GameLog},
    player::Player,
    AppState,
};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, show_hud.run_if(in_state(AppState::InGame)));
    }
}

fn show_hud(
    q_stats: Query<&CombatStats, With<Player>>,
    game_log: Res<GameLog>,
    mut contexts: EguiContexts,
) {
    let stats = q_stats.single();

    let length = game_log.entries.len();

    let mut logs = vec![];

    if length <= 4 {
        for i in 0..length {
            logs.push(game_log.entries[i].clone());
        }
    } else {
        for i in length - 4..length {
            logs.push(game_log.entries[i].clone());
        }
    }

    egui::TopBottomPanel::bottom("my_bottom")
        .min_height(100.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.columns(2, |columns| {
                egui::ScrollArea::vertical().show(&mut columns[0], |ui| {
                    for log in logs.iter() {
                        ui.label(log);
                    }
                });

                egui::Frame::none().show(&mut columns[1], |ui| {
                    ui.horizontal(|ui| {
                        let progress = stats.hp as f32 / stats.max_hp as f32;

                        ui.label(format!("{}/{}", stats.hp, stats.max_hp));

                        ui.add(egui::ProgressBar::new(progress));
                    });
                });
            });
        });
}
