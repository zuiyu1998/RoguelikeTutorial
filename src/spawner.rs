use bevy::{
    asset::Assets,
    core::Name,
    ecs::{
        entity::Entity,
        system::{Commands, ResMut, SystemParam},
    },
    hierarchy::BuildChildren,
    sprite::TextureAtlasLayout,
};

use crate::{
    common::{CombatStats, Position, RandomNumberGenerator, Viewshed},
    consts::{ENEMY_Z_INDEX, ITEM_Z_INDEX, PLAYER_Z_INDEX},
    core::TextureAssets,
    enemy::{add_state_machine, Enemy, EnemyType},
    item::{Consumable, InflictsDamage, Item, ItemType, ProvidesHealing, Ranged},
    map::{BlocksTile, Rect},
    player::Player,
    render::create_sprite_sheet_bundle,
    theme::Theme,
};

#[derive(SystemParam)]
pub struct ThemeContext<'w> {
    pub texture_assets: ResMut<'w, TextureAssets>,
    pub layout_assets: ResMut<'w, Assets<TextureAtlasLayout>>,
    pub theme: ResMut<'w, Theme>,
}

pub fn magic_missile_scroll(
    commands: &mut Commands,
    theme_context: &mut ThemeContext,
    x: i32,
    y: i32,
) -> Entity {
    let mut sprite_bundle = create_sprite_sheet_bundle(
        &theme_context.texture_assets,
        &mut theme_context.layout_assets,
        theme_context
            .theme
            .item_to_render(ItemType::MagicMissileScroll),
    );
    sprite_bundle.transform.translation.z = ITEM_Z_INDEX;

    commands
        .spawn((
            sprite_bundle,
            Position { x, y },
            Name::new("Magic Missile Scroll"),
            Item {},
            ProvidesHealing { heal_amount: 10 },
            Consumable {},
            Ranged { range: 6 },
            InflictsDamage { damage: 8 },
            ItemType::MagicMissileScroll,
        ))
        .id()
}

pub fn health_potion(
    commands: &mut Commands,
    theme_context: &mut ThemeContext,
    x: i32,
    y: i32,
) -> Entity {
    let mut sprite_bundle = create_sprite_sheet_bundle(
        &theme_context.texture_assets,
        &mut theme_context.layout_assets,
        theme_context.theme.item_to_render(ItemType::HealthPotion),
    );
    sprite_bundle.transform.translation.z = ITEM_Z_INDEX;

    commands
        .spawn((
            sprite_bundle,
            Position { x, y },
            Name::new("Item"),
            Item {},
            ProvidesHealing { heal_amount: 10 },
            Consumable {},
            ItemType::HealthPotion,
        ))
        .id()
}

pub fn spawn_room(
    commands: &mut Commands,
    theme_context: &mut ThemeContext,
    map_entity: Entity,
    rng: &mut RandomNumberGenerator,
    room: &Rect,
    room_index: usize,
    max_enemy: usize,
    max_item: usize,
) {
    let mut monster_spawn_points: Vec<Position> = Vec::new();
    let mut item_spawn_points: Vec<Position> = Vec::new();

    let num_monsters = rng.roll_dice(1, max_enemy as i32 + 2) - 3;
    let num_items = rng.roll_dice(1, max_item as i32 + 2) - 3;

    for _i in 0..num_monsters {
        let mut added = false;
        while !added {
            let x = room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1));
            let y = room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1));

            let pos = Position { x, y };

            if !monster_spawn_points.contains(&pos) {
                monster_spawn_points.push(pos);
                added = true;
            }
        }
    }

    for _i in 0..num_items {
        let mut added = false;
        while !added {
            let x = room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1));
            let y = room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1));
            let pos = Position { x, y };

            if !item_spawn_points.contains(&pos) {
                item_spawn_points.push(pos);
                added = true;
            }
        }
    }

    for (room_enemy_index, pos) in monster_spawn_points.iter().enumerate() {
        let enemy_index = room_index * max_enemy + room_enemy_index;

        let enemy = random_enemy(commands, theme_context, rng, pos.x, pos.y, enemy_index);

        commands.entity(enemy).set_parent(map_entity);
    }

    for pos in item_spawn_points.iter() {
        let item_entity = random_item(commands, theme_context, rng, pos.x, pos.y);

        commands.entity(item_entity).set_parent(map_entity);
    }
}

pub fn player(commands: &mut Commands, theme_context: &mut ThemeContext, x: i32, y: i32) -> Entity {
    let mut sprite_bundle = create_sprite_sheet_bundle(
        &theme_context.texture_assets,
        &mut theme_context.layout_assets,
        theme_context.theme.player_to_render(),
    );
    sprite_bundle.transform.translation.z = PLAYER_Z_INDEX;

    commands
        .spawn((
            sprite_bundle,
            Position { x, y },
            Player,
            Viewshed {
                range: 9,
                visible_tiles: vec![],
                dirty: true,
            },
            Name::new("Player"),
            CombatStats {
                max_hp: 30,
                hp: 30,
                defense: 2,
                power: 5,
            },
        ))
        .id()
}

pub fn enemy(
    commands: &mut Commands,
    theme_context: &mut ThemeContext,
    enemy_tile: EnemyType,
    name: &str,
    x: i32,
    y: i32,
) -> Entity {
    let mut sprite_bundle = create_sprite_sheet_bundle(
        &theme_context.texture_assets,
        &mut theme_context.layout_assets,
        theme_context.theme.enemy_to_render(enemy_tile),
    );

    sprite_bundle.transform.translation.z = ENEMY_Z_INDEX;

    let monster = commands
        .spawn((
            sprite_bundle,
            Position { x, y },
            Enemy,
            Viewshed {
                range: 9,
                visible_tiles: vec![],
                dirty: true,
            },
            Name::new(name.to_owned()),
            BlocksTile,
            CombatStats {
                max_hp: 16,
                hp: 16,
                defense: 1,
                power: 3,
            },
        ))
        .id();

    add_state_machine(&mut commands.entity(monster), enemy_tile);

    monster
}

pub fn goblin(
    commands: &mut Commands,
    theme_context: &mut ThemeContext,
    name: &str,
    x: i32,
    y: i32,
) -> Entity {
    let monster = enemy(commands, theme_context, EnemyType::G, name, x, y);

    monster
}

pub fn orc(
    commands: &mut Commands,
    theme_context: &mut ThemeContext,
    name: &str,
    x: i32,
    y: i32,
) -> Entity {
    let monster = enemy(commands, theme_context, EnemyType::O, name, x, y);

    monster
}

pub fn random_item(
    commands: &mut Commands,
    theme_context: &mut ThemeContext,
    rng: &mut RandomNumberGenerator,
    x: i32,
    y: i32,
) -> Entity {
    let roll: i32;
    {
        roll = rng.roll_dice(1, 2);
    }

    match roll {
        1 => {
            return health_potion(commands, theme_context, x, y);
        }

        __ => {
            return magic_missile_scroll(commands, theme_context, x, y);
        }
    }
}

pub fn random_enemy(
    commands: &mut Commands,
    theme_context: &mut ThemeContext,
    rng: &mut RandomNumberGenerator,
    x: i32,
    y: i32,
    i: usize,
) -> Entity {
    let roll: i32;
    {
        roll = rng.roll_dice(1, 2);
    }

    match roll {
        1 => {
            let name = format!("Goblin #{}", i);

            return goblin(commands, theme_context, &name, x, y);
        }

        __ => {
            let name = format!("Orc #{}", i);

            return orc(commands, theme_context, &name, x, y);
        }
    }
}
