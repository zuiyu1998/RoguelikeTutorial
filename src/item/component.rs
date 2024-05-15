use bevy::prelude::*;

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
