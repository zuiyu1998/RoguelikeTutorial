use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContexts,
};

use crate::{state::AppStateManager, AppState};

pub struct MenuUiState {
    item_list: Vec<MenuItem>,
}

impl Default for MenuUiState {
    fn default() -> Self {
        MenuUiState {
            item_list: vec![MenuItem::playing()],
        }
    }
}

#[derive(Debug, Clone, Event)]
pub struct MenuItem {
    item_type: MenuItemType,
}

impl MenuItem {
    pub fn playing() -> Self {
        MenuItem {
            item_type: MenuItemType::Playing,
        }
    }
}

#[derive(Debug, Clone)]
pub enum MenuItemType {
    Playing,
}

impl ToString for MenuItemType {
    fn to_string(&self) -> String {
        match *self {
            MenuItemType::Playing => format!("Playing"),
        }
    }
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MenuItem>();

        app.add_systems(
            Update,
            (show_menu, handle_menu_item).run_if(in_state(AppState::Menu)),
        );
    }
}

fn handle_menu_item(
    mut menu_item_er: EventReader<MenuItem>,
    mut app_state_manager: AppStateManager,
) {
    for e in menu_item_er.read() {
        match e.item_type {
            MenuItemType::Playing => {
                app_state_manager.start_game();
            }
        }
    }
}

fn show_menu(mut contexts: EguiContexts, mut menu_item_ew: EventWriter<MenuItem>) {
    let ui_state = MenuUiState::default();

    egui::Window::new("menu")
        .title_bar(false)
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .show(contexts.ctx_mut(), |ui| {
            for item in ui_state.item_list.iter() {
                let button = ui.add(egui::Button::new(item.item_type.to_string()));

                if button.clicked() {
                    menu_item_ew.send(item.clone());
                }
            }
        });
}
