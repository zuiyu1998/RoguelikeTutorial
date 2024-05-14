mod backpack;
mod hub;
mod player;
mod tooltip;

pub use backpack::*;
use bevy::prelude::*;
use hub::HudPlugin;

use player::PlayerUIPlugin;
use tooltip::TooltipsPlugin;

use crate::{state::AppStateManager, GameState};

pub struct InternalUiPlugin;

impl Plugin for InternalUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((HudPlugin, TooltipsPlugin, PlayerUIPlugin));

        app.add_systems(Update, show_tab.run_if(in_state(GameState::Playing)));
        app.add_systems(Update, close_tab.run_if(in_state(GameState::Tab)));
    }
}

fn show_tab(keyboard_input: Res<ButtonInput<KeyCode>>, mut app_state_manager: AppStateManager) {
    if keyboard_input.just_pressed(KeyCode::Tab) {
        app_state_manager.start_tab();
    }
}

fn close_tab(keyboard_input: Res<ButtonInput<KeyCode>>, mut app_state_manager: AppStateManager) {
    if keyboard_input.just_pressed(KeyCode::Tab) {
        app_state_manager.start_playing();
    }
}
