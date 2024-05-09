use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub struct CoreUiPlugin;

impl Plugin for CoreUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, configure_visuals_system);

        #[cfg(not(feature = "dev"))]
        {
            use bevy_egui::EguiPlugin;
            app.add_plugins((EguiPlugin));
        }
    }
}

fn configure_visuals_system(mut contexts: EguiContexts) {
    contexts.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        window_shadow: egui::epaint::Shadow::NONE,
        ..Default::default()
    });
}
