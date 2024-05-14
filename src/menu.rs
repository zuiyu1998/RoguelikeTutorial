use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_egui::egui::{self, Align2};

use crate::{core::prelude::*, state::AppStateManager, AppState};

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

#[derive(SystemParam)]
pub struct MenuUiParams<'w> {
    pub menu_item_er: EventWriter<'w, MenuItem>,
}

impl<'w> UiSystem for MenuUiParams<'w>
where
    'w: 'static,
{
    type UiState = MenuUiState;

    fn extra_ui_state(
        _item: &<Self as bevy::ecs::system::SystemParam>::Item<'_, '_>,
    ) -> Self::UiState {
        MenuUiState::default()
    }
}

impl<'w> UiContainer<MenuUiParams<'w>> for MenuUiState
where
    'w: 'static,
{
    fn container(
        &self,
        ui_context: EguiUiContext,
        mut bevy_context: BevyBuildContext<
            <MenuUiParams<'w> as bevy::ecs::system::SystemParam>::Item<'_, '_>,
        >,
    ) {
        egui::Window::new("menu")
            .title_bar(false)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ui_context.get(), |ui| {
                for item in self.item_list.iter() {
                    let button = ui.add(egui::Button::new(item.item_type.to_string()));

                    if button.clicked() {
                        bevy_context.item.menu_item_er.send(item.clone());
                    }
                }
            });
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
            (MenuUiParams::show_ui, handle_menu_item).run_if(in_state(AppState::Menu)),
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
