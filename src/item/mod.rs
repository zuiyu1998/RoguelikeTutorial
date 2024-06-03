mod component;

use bevy::{prelude::*, utils::HashMap};

use crate::{
    common::{CombatStats, GameLog, Position},
    core::TextureAssets,
    enemy::Enemy,
    map::Map,
    AppState,
};

pub use component::*;

//计算道具影响的地点或者实体
fn compute_item_apply_position_or_entity(
    mut commands: Commands,
    mut q_wants_use_item: Query<(&Parent, &WantsToUseItem, Entity), Without<ItemTargetEntity>>,
    q_items: Query<
        (
            &ItemTargetType,
            Option<&ItemUseStartPosition>,
            Option<&Ranged>,
        ),
        (With<Item>, With<InBackpack>),
    >,
    q_enemy: Query<&Position, With<Enemy>>,
    q_character: Query<&Position>,
    map: Res<Map>,
) {
    for (parent, wants_use_item, entity) in q_wants_use_item.iter_mut() {
        if let Ok((item_target_type, start_position, ranged)) = q_items.get(wants_use_item.item) {
            match item_target_type {
                ItemTargetType::Owner => {
                    commands
                        .entity(entity)
                        .insert(ItemTargetEntity(vec![parent.get()]));
                }
                ItemTargetType::Computed(computed_type) => {
                    let ranged = ranged.unwrap();

                    let start_positon: Position = match start_position {
                        Some(position) => position.0,
                        None => {
                            let position = q_character.get(parent.get()).unwrap();

                            *position
                        }
                    };

                    if let Some(enemys) =
                        map.get_all_enemy(&start_positon, ranged.range, ranged.range)
                    {
                        match computed_type {
                            &ItemTargetComputedType::Entity => {
                                commands.entity(entity).insert(ItemTargetEntity(enemys));
                            }
                            &ItemTargetComputedType::Area => {
                                let positons = enemys
                                    .iter()
                                    .map(|entity| {
                                        let position = q_enemy.get(*entity).unwrap();

                                        *position
                                    })
                                    .collect();

                                commands.entity(entity).insert(ItemTargetPosition(positons));
                            }
                        }
                    } else {
                        //todo 提示道具使用失败

                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        }
    }
}

//使用生命药水
fn item_use_healing(
    q_wants_use_item: Query<(&Parent, &WantsToUseItem, Entity, &ItemTargetEntity)>,
    mut q_stats: Query<&mut CombatStats>,
    mut q_items: Query<(Option<&Consumable>, &ProvidesHealing), (With<Item>, With<InBackpack>)>,
    mut item_remove_ew: EventWriter<ItemRemoveEvent>,
    mut commands: Commands,
) {
    for (parent, wants_use_item, entity, item_target_entity) in q_wants_use_item.iter() {
        let (consuable, healing) = q_items.get_mut(wants_use_item.item).unwrap();

        for item_target in item_target_entity.0.iter() {
            if let Ok(mut stats) = q_stats.get_mut(*item_target) {
                let tmp_hp = stats.hp + healing.heal_amount;

                stats.hp = tmp_hp.min(stats.max_hp);
            }
        }

        if let Some(_) = consuable {
            commands.entity(entity).despawn_recursive();

            item_remove_ew.send(ItemRemoveEvent {
                owner: parent.get(),
                item: wants_use_item.item,
            });
        }
    }
}

#[derive(Component, Debug)]
pub struct InflictsDamage {
    pub damage: i32,
}

fn item_on_start_game(mut commands: Commands) {
    commands.init_resource::<ItemInBackpacks>();
}

fn item_on_end_game(mut commands: Commands) {
    commands.remove_resource::<ItemInBackpacks>();
}

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ItemPickUpEvent>();
        app.add_event::<ItemRemoveEvent>();
        app.add_event::<ItemApplyEvent>();

        app.add_systems(OnEnter(AppState::InGame), item_on_start_game);
        app.add_systems(OnExit(AppState::InGame), item_on_end_game);

        app.add_systems(
            Update,
            (
                item_collect,
                handle_item_update_event,
                handle_item_apply_event,
                item_use_healing,
                compute_item_apply_position_or_entity,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}

//拾取事件
#[derive(Debug, Event)]
pub struct ItemPickUpEvent {
    owner: Entity,
    item: Entity,
}

//道具删除事件
#[derive(Debug, Event)]
pub struct ItemRemoveEvent {
    item: Entity,
    owner: Entity,
}

//道具使用事件
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

pub fn handle_item_apply_event(
    mut item_apply_er: EventReader<ItemApplyEvent>,
    mut commands: Commands,
) {
    for event in item_apply_er.read() {
        commands.entity(event.owner).with_children(|parent| {
            parent.spawn(WantsToUseItem { item: event.item });
        });
    }
}

pub fn handle_item_update_event(
    mut item_remove_er: EventReader<ItemRemoveEvent>,
    mut item_added_er: EventReader<ItemPickUpEvent>,
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
            let item_data = item_in_back.remove(item_type);

            if item_data.is_none() {
                continue;
            }

            let mut item_data = item_data.unwrap();

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
    q_items: Query<&Name, (With<Item>, Without<InBackpack>)>,
    mut item_ew: EventWriter<ItemPickUpEvent>,
    mut game_log: ResMut<GameLog>,
) {
    for (parent, wants_to_pickup_item_entity, wants_to_pickup_item) in q_wants_to_pickup_item.iter()
    {
        if let Ok(name) = q_items.get(wants_to_pickup_item.item) {
            commands
                .entity(wants_to_pickup_item.item)
                .insert(InBackpack {
                    owner: parent.get(),
                })
                .remove::<SpriteSheetBundle>()
                .remove::<Position>()
                .set_parent(wants_to_pickup_item.collected_by);

            game_log.entries.push(format!("You pick up the {}.", name));

            item_ew.send(ItemPickUpEvent {
                owner: parent.get(),
                item: wants_to_pickup_item.item,
            });

            commands
                .entity(wants_to_pickup_item_entity)
                .despawn_recursive();
        }
    }
}

//标记拾取的组件
#[derive(Component, Debug, Clone)]
#[component(storage = "SparseSet")]
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
    MagicMissileScroll,
}

impl ItemType {
    pub fn get_image_handle(&self, texture_assets: &TextureAssets) -> Handle<Image> {
        match self {
            ItemType::HealthPotion => texture_assets.i.clone(),
            ItemType::MagicMissileScroll => texture_assets.i.clone(),
        }
    }
}
