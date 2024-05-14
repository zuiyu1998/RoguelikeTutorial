use super::{BackPackUiState, BackPackUiStateItem, ItemUiData};
use crate::{
    core::{
        BevyBuildContext, BuildUiWidght, EguiUiContext, EguiWidghtBuildContext, TextureAssets,
        UiContainer, UiSystem, UiWidght,
    },
    item::{ItemApplyEvent, ItemInBackpacks},
    player::PlayerEntity,
    GameState,
};
use bevy::{ecs::system::SystemParam, prelude::*};

use bevy_egui::egui;

#[derive(Default)]
pub struct PlayerUiState {
    backpack: BackPackUiState,
}

pub struct PlayerUIPlugin;

impl Plugin for PlayerUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            PlayerUIParams::show_ui.run_if(in_state(GameState::Tab)),
        );
    }
}

#[derive(SystemParam)]
pub struct PlayerUIParams<'w> {
    q_items: Res<'w, ItemInBackpacks>,
    player_entity: Res<'w, PlayerEntity>,
    texture_assets: Res<'w, TextureAssets>,
    item_ew: EventWriter<'w, ItemApplyEvent>,
}

impl<'b, 'w> BuildUiWidght<BackPackUiStateItem<'b, 'w>, PlayerUIParams<'w>> for PlayerUiState
where
    'w: 'static,
{
    fn build_widght(
        &self,
        ui_context: &EguiUiContext,
        bevy_context: &mut BevyBuildContext<<PlayerUIParams<'w> as SystemParam>::Item<'_, '_>>,
        ui: &mut egui::Ui,
    ) {
        //todo 简化
        let ui_state_item = BackPackUiStateItem {
            item_ew: &mut bevy_context.item.item_ew,
        };
        let widght_build_context = EguiWidghtBuildContext::new(ui_state_item, ui_context);
        self.backpack.widght(widght_build_context, ui);
    }
}

impl<'w: 'static> UiContainer<PlayerUIParams<'w>> for PlayerUiState {
    fn container(
        &self,
        ui_context: crate::core::EguiUiContext,
        mut bevy_context: BevyBuildContext<<PlayerUIParams<'w> as SystemParam>::Item<'_, '_>>,
    ) {
        egui::Window::new("背包").show(ui_context.get(), |ui| {
            self.build_widght(&ui_context, &mut bevy_context, ui);
        });
    }
}

impl<'w: 'static> UiSystem for PlayerUIParams<'w> {
    type UiState = PlayerUiState;

    fn extra_ui_state(item: &<Self as SystemParam>::Item<'_, '_>) -> Self::UiState {
        let player_entity = item.player_entity.0;

        let mut state = PlayerUiState::default();

        if let Some(item_in_back) = item.q_items.get(&player_entity) {
            if item_in_back.len() > state.backpack.data.len() {
                let mut tmp = vec![];
                for (item_type, item_data) in item_in_back.iter() {
                    tmp.push(ItemUiData::new(
                        item_data.clone(),
                        item_type.get_image_handle(&item.texture_assets),
                        *item_type,
                        player_entity,
                    ));
                }

                state.backpack.data = tmp;
            } else {
                for (index, (item_type, item_data)) in item_in_back.iter().enumerate() {
                    state.backpack.data[index] = ItemUiData::new(
                        item_data.clone(),
                        item_type.get_image_handle(&item.texture_assets),
                        *item_type,
                        player_entity,
                    );
                }
            }
        }

        state
    }
}
