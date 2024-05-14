use bevy::prelude::*;

use super::{Item, ItemApplyEvent, ItemComponent, ItemType, ItemTypePlugin};
use crate::{common::CombatStats, AppState};

impl ItemComponent for Potion {
    type Event = PotionEvent;

    fn from_item_apply_event(event: &ItemApplyEvent) -> Option<Self::Event> {
        if matches!(event.item_type, ItemType::HealthPotion) {
            Some(PotionEvent { item: event.item })
        } else {
            None
        }
    }
}

//生命药水
#[derive(Component, Debug)]
pub struct Potion {
    pub heal_amount: i32,
}

#[derive(Debug, Event)]
pub struct PotionEvent {
    item: Entity,
}

fn potion_apply(
    mut item_er: EventReader<PotionEvent>,
    q_items: Query<(&Parent, &Potion), With<Item>>,
    mut q_stats: Query<&mut CombatStats>,
    mut commands: Commands,
) {
    for e in item_er.read() {
        if let Ok((parent, potion)) = q_items.get(e.item) {
            if let Ok(mut stats) = q_stats.get_mut(parent.get()) {
                let tmp_hp = stats.hp + potion.heal_amount;

                stats.hp = tmp_hp.min(stats.max_hp);

                commands.entity(e.item).despawn_recursive();
            }
        }
    }
}

pub struct PotionPlugin;

impl Plugin for PotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ItemTypePlugin::<Potion>::default());

        app.add_systems(Update, (potion_apply,).run_if(in_state(AppState::InGame)));
    }
}
