mod potion;

use bevy::{prelude::*, utils::HashMap};
use std::marker::PhantomData;

use crate::{common::Position, core::TextureAssets, AppState};

pub use potion::*;

fn item_on_start_game(mut commands: Commands) {
    commands.init_resource::<ItemInBackpacks>();
}

fn item_on_end_game(mut commands: Commands) {
    commands.remove_resource::<ItemInBackpacks>();
}

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PotionPlugin,));

        app.add_event::<ItemAddedEvent>();
        app.add_event::<ItemApplyEvent>();
        app.add_event::<ItemRemoveEvent>();

        app.add_systems(OnEnter(AppState::InGame), item_on_start_game);
        app.add_systems(OnExit(AppState::InGame), item_on_end_game);

        app.add_systems(
            Update,
            (item_collect, handle_item_update_event).run_if(in_state(AppState::InGame)),
        );
    }
}

pub struct ItemTypePlugin<Item>(PhantomData<Item>);

impl<Item> Default for ItemTypePlugin<Item> {
    fn default() -> Self {
        ItemTypePlugin(PhantomData::default())
    }
}

impl<Item> Plugin for ItemTypePlugin<Item>
where
    Item: ItemComponent,
{
    fn build(&self, app: &mut App) {
        app.add_event::<Item::Event>();

        app.add_systems(
            Update,
            (handle_item_apply_event::<Item>).run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Debug, Event)]
pub struct ItemAddedEvent {
    owner: Entity,
    item: Entity,
}

#[derive(Debug, Event)]
pub struct ItemRemoveEvent {
    item: Entity,
    owner: Entity,
}

#[derive(Debug, Event)]
pub struct ItemApplyEvent {
    pub item: Entity,
    pub item_type: ItemType,
    pub owner: Entity,
}

#[derive(Debug, Clone, Default)]
pub struct ItemData {
    pub count: i32,
    pub data: Vec<Entity>,
}

#[derive(Debug, Resource, Deref, DerefMut, Default)]
pub struct ItemInBackpacks(HashMap<Entity, ItemInBackpack>);

#[derive(Debug, Resource, Deref, DerefMut, Default)]
pub struct ItemInBackpack(HashMap<ItemType, ItemData>);

pub fn handle_item_apply_event<Item>(
    mut item_apply_er: EventReader<ItemApplyEvent>,
    mut item_ew: EventWriter<Item::Event>,
    mut item_reomve_ew: EventWriter<ItemRemoveEvent>,
) where
    Item: ItemComponent,
{
    let mut events = vec![];

    for e in item_apply_er.read() {
        if let Some(event) = Item::from_item_apply_event(e) {
            events.push((
                ItemRemoveEvent {
                    owner: e.owner,
                    item: e.item,
                },
                event,
            ));
        }
    }

    for (remove_event, item_event) in events.into_iter() {
        item_ew.send(item_event);

        if Item::once() {
            item_reomve_ew.send(remove_event);
        }
    }
}

pub fn handle_item_update_event(
    mut item_remove_er: EventReader<ItemRemoveEvent>,
    mut item_added_er: EventReader<ItemAddedEvent>,
    mut item_in_backs: ResMut<ItemInBackpacks>,
    mut q_item: Query<&ItemType>,
) {
    for event in item_remove_er.read() {
        let mut item_in_back = match item_in_backs.remove(&event.owner) {
            None => {
                continue;
            }
            Some(item_in_back) => item_in_back,
        };

        let mut need_insert = true;

        if let Ok(item_type) = q_item.get_mut(event.item) {
            let mut item_data = item_in_back.remove(item_type).unwrap_or_default();
            item_data.count -= 1;
            item_data.data.pop();

            if item_data.count > 0 {
                item_in_back.insert(*item_type, item_data);
            }

            if item_in_back.is_empty() {
                need_insert = false;
            }
        }

        if need_insert {
            item_in_backs.insert(event.owner, item_in_back);
        }
    }

    for event in item_added_er.read() {
        let mut item_in_back = item_in_backs.remove(&event.owner).unwrap_or_default();

        if let Ok(item_type) = q_item.get_mut(event.item) {
            let mut item_data = item_in_back.remove(item_type).unwrap_or_default();
            item_data.count += 1;
            item_data.data.push(event.item);

            item_in_back.insert(*item_type, item_data);
        }

        item_in_backs.insert(event.owner, item_in_back);
    }
}

pub fn item_collect(
    mut commands: Commands,
    q_wants_to_pickup_item: Query<(&Parent, Entity, &WantsToPickupItem)>,
    q_items: Query<Entity, (With<Item>, Without<InBackpack>)>,
    mut item_ew: EventWriter<ItemAddedEvent>,
) {
    for (parent, wants_to_pickup_item_entity, wants_to_pickup_item) in q_wants_to_pickup_item.iter()
    {
        if let Ok(_) = q_items.get(wants_to_pickup_item.item) {
            commands
                .entity(wants_to_pickup_item.item)
                .insert(InBackpack {
                    owner: parent.get(),
                })
                .remove::<SpriteSheetBundle>()
                .remove::<Position>()
                .set_parent(wants_to_pickup_item.collected_by);

            item_ew.send(ItemAddedEvent {
                owner: parent.get(),
                item: wants_to_pickup_item.item,
            });

            commands
                .entity(wants_to_pickup_item_entity)
                .despawn_recursive();
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct WantsToPickupItem {
    pub collected_by: Entity,
    pub item: Entity,
}

#[derive(Component, Debug, Clone)]
pub struct InBackpack {
    pub owner: Entity,
}

#[derive(Component, Debug)]
pub struct Item;

#[derive(Debug, Component, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ItemType {
    HealthPotion,
}

impl ItemType {
    pub fn get_image_handle(&self, texture_assets: &TextureAssets) -> Handle<Image> {
        match self {
            ItemType::HealthPotion => texture_assets.i.clone(),
        }
    }
}

pub trait ItemComponent: Component {
    type Event: Event;

    fn from_item_apply_event(event: &ItemApplyEvent) -> Option<Self::Event>;

    fn once() -> bool {
        true
    }
}
