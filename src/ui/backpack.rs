use bevy::prelude::*;
use bevy_egui::egui;

use crate::{
    core::{EguiWidghtBuildContext, UiWidght},
    item::{ItemApplyEvent, ItemData, ItemType},
};

pub struct BackPackUiState {
    pub data: Vec<ItemUiData>,
    pub row_count: usize,
}

pub struct BackPackUiStateItem<'b, 'w> {
    pub item_ew: &'b mut EventWriter<'w, ItemApplyEvent>,
}

impl<'b, 'w> UiWidght<BackPackUiStateItem<'b, 'w>> for BackPackUiState {
    fn widght<'a>(
        &self,
        context: EguiWidghtBuildContext<'a, BackPackUiStateItem<'a, 'w>>,
        ui: &'a mut egui::Ui,
    ) {
        let len = self.data.len();
        let step = self.row_count;

        let EguiWidghtBuildContext {
            mut item,
            ui_context,
        } = context;

        for (index, _) in (0..len).into_iter().step_by(step).enumerate() {
            ui.columns(step, |columns: &mut [egui::Ui]| {
                for column_index in 0..step {
                    let data_index = index * 9 + column_index;

                    let ui_state_item = BackPackUiStateItem {
                        item_ew: &mut item.item_ew,
                    };

                    let widght_build_context =
                        EguiWidghtBuildContext::new(ui_state_item, ui_context);

                    self.data[data_index].widght(widght_build_context, &mut columns[column_index]);
                }
            });
        }
    }
}

impl Default for BackPackUiState {
    fn default() -> Self {
        BackPackUiState {
            data: vec![ItemUiData::default(); 45],
            row_count: 9,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ItemUiData(Option<ItemUiDataInternal>);

#[derive(Debug, Clone)]
pub struct ItemUiDataInternal {
    pub item_data: ItemData,
    pub item_image: Handle<Image>,
    pub item_type: ItemType,
    pub owner: Entity,
}

impl ItemUiData {
    pub fn new(
        item_data: ItemData,
        item_image: Handle<Image>,
        item_type: ItemType,
        owner: Entity,
    ) -> Self {
        ItemUiData(Some(ItemUiDataInternal {
            item_data,
            item_image,
            item_type,
            owner,
        }))
    }

    pub fn get_item_data(&self) -> Option<&ItemData> {
        self.0
            .as_ref()
            .and_then(|internal| Some(&internal.item_data))
    }

    pub fn get_item_image(&self) -> Option<&Handle<Image>> {
        self.0
            .as_ref()
            .and_then(|internal| Some(&internal.item_image))
    }

    pub fn get_item(&self) -> Option<&ItemUiDataInternal> {
        self.0.as_ref()
    }
}

impl<'b, 'w> UiWidght<BackPackUiStateItem<'b, 'w>> for ItemUiData {
    fn widght<'a>(
        &self,
        context: EguiWidghtBuildContext<'a, BackPackUiStateItem<'b, 'w>>,
        ui: &'a mut egui::Ui,
    ) {
        ui.set_height(60.0);
        ui.set_width(60.0);

        let frame = egui::Frame::none();

        frame.show(ui, |ui| {
            if self.0.is_some() {
                let item_image_id = context.image_id(self.get_item_image().unwrap()).unwrap();

                let rect = ui.available_rect_before_wrap();

                let mut child_ui = ui.child_ui(
                    rect,
                    egui::Layout::centered_and_justified(egui::Direction::RightToLeft),
                );

                child_ui.image(egui::load::SizedTexture::new(
                    item_image_id,
                    egui::vec2(32., 32.),
                ));

                let rect = ui.available_rect_before_wrap();

                let mut child_ui = ui.child_ui(rect, egui::Layout::bottom_up(egui::Align::Max));

                child_ui.label(format!("{}", self.get_item_data().unwrap().count));
            }
        });

        let size = ui.available_size();

        let (_, res) = ui.allocate_exact_size(size, egui::Sense::click());

        if res.clicked() {
            if let Some(item) = self.get_item() {
                context.item.item_ew.send(ItemApplyEvent {
                    item: *item.item_data.data.last().unwrap(),
                    item_type: item.item_type,
                    owner: item.owner,
                });
            }
        }
    }
}
