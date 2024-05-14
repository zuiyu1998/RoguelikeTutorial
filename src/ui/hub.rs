use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_egui::egui;

use crate::{
    common::{CombatStats, GameLog},
    core::prelude::*,
    player::Player,
    AppState,
};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            HudParams::show_ui.run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(SystemParam)]
pub struct HudParams<'w, 's> {
    q_stats: Query<'w, 's, &'static CombatStats, With<Player>>,
    game_log: Res<'w, GameLog>,
}

impl<'w, 's> UiSystem for HudParams<'w, 's>
where
    'w: 'static,
    's: 'static,
{
    type UiState = HudUiState;

    fn extra_ui_state(item: &<Self as SystemParam>::Item<'_, '_>) -> Self::UiState {
        let stats = item.q_stats.single();

        let length = item.game_log.entries.len();

        let mut logs = vec![];

        if length <= 4 {
            for i in 0..length {
                logs.push(item.game_log.entries[i].clone());
            }
        } else {
            for i in length - 4..length {
                logs.push(item.game_log.entries[i].clone());
            }
        }

        HudUiState {
            logs,
            hp: stats.hp,
            max_hp: stats.max_hp,
        }
    }
}

pub struct HudUiState {
    logs: Vec<String>,
    hp: i32,
    max_hp: i32,
}

impl<'w, 's> UiContainer<HudParams<'w, 's>> for HudUiState
where
    'w: 'static,
    's: 'static,
{
    fn container(
        &self,
        ui_context: EguiUiContext,
        _bevy_context: BevyBuildContext<<HudParams<'w, 's> as SystemParam>::Item<'_, '_>>,
    ) {
        egui::TopBottomPanel::bottom("my_bottom")
            .min_height(100.0)
            .show(ui_context.get(), |ui| {
                ui.columns(2, |columns| {
                    egui::ScrollArea::vertical().show(&mut columns[0], |ui| {
                        for log in self.logs.iter() {
                            ui.label(log);
                        }
                    });

                    egui::Frame::none().show(&mut columns[1], |ui| {
                        ui.horizontal(|ui| {
                            let progress = self.hp as f32 / self.max_hp as f32;

                            ui.label(format!("{}/{}", self.hp, self.max_hp));

                            ui.add(egui::ProgressBar::new(progress));
                        });
                    });
                });
            });
    }
}
