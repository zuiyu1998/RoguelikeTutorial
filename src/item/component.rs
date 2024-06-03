use bevy::prelude::*;

use crate::common::Position;

//道具使用的位置
#[derive(Component, Debug)]
pub struct ItemUseStartPosition(pub Position);

//获取itemTarget的方式
#[derive(Component, Debug)]
pub enum ItemTargetType {
    Computed(ItemTargetComputedType),
    Owner,
}

#[derive(Debug)]
pub enum ItemTargetComputedType {
    Area,
    Entity,
}

//道具目标
#[derive(Component, Debug)]
pub struct ItemTargetEntity(pub Vec<Entity>);

#[derive(Component, Debug)]
pub struct ItemTargetPosition(pub Vec<Position>);

//道具范围
#[derive(Component, Debug)]
pub struct Ranged {
    pub range: i32,
}

//item 使用组件
#[derive(Component, Debug)]
#[component(storage = "SparseSet")]
pub struct WantsToUseItem {
    pub item: Entity,
}

//item是否是一次性的
#[derive(Component, Debug)]
pub struct Consumable {}

//生命药水
#[derive(Component, Debug)]
pub struct ProvidesHealing {
    pub heal_amount: i32,
}
