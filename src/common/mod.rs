use crate::{
    consts::SPRITE_SIZE,
    map::{Map, MapTile},
    player::PlayerEntity,
    theme::Theme,
    GameState,
};
use bevy::{prelude::*, utils::hashbrown::HashMap};
use bracket_pathfinding::prelude::{field_of_view, Point};
use bracket_random::prelude::RandomNumberGenerator as BracketRandomNumberGenerator;

#[derive(Resource, Deref, DerefMut)]
pub struct RandomNumberGenerator(BracketRandomNumberGenerator);

#[derive(Resource, Default)]
pub struct GameLog {
    pub entries: Vec<String>,
}

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component, Reflect, PartialEq, Eq, Debug)]
#[reflect(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub fn delete_the_dead(
    mut commands: Commands,
    q_combat_stats: Query<(&CombatStats, Entity, &Name)>,
    player_entity: Res<PlayerEntity>,
    mut next_state: ResMut<NextState<GameState>>,
    mut log: ResMut<GameLog>,
) {
    for (combat_stats, entity, name) in q_combat_stats.iter() {
        if combat_stats.hp <= 0 {
            if entity == player_entity.0 {
                next_state.set(GameState::Menu);
            } else {
                commands.entity(entity).despawn_recursive();

                log.entries.push(format!("{} is dead", &name));
            }
        }
    }
}

pub fn apply_damage(
    mut commands: Commands,
    mut q_suffer_damage: Query<(&mut CombatStats, &SufferDamage, Entity)>,
) {
    for (mut stats, damage, entity) in q_suffer_damage.iter_mut() {
        stats.hp -= damage.amount.iter().sum::<i32>();

        commands.entity(entity).remove::<SufferDamage>();
    }
}

pub fn melee_combat(
    mut commands: Commands,
    q_wants_to_melee: Query<(&WantsToMelee, &Parent, Entity)>,
    mut q_combat_stats: Query<(&CombatStats, &Name, Option<&mut SufferDamage>)>,
    mut log: ResMut<GameLog>,
) {
    let mut damage_map: HashMap<Entity, Vec<i32>> = HashMap::default();

    for (wants_to_melee, parent, entity) in q_wants_to_melee.iter() {
        let (active, active_name, _) = q_combat_stats.get(parent.get()).unwrap();
        if active.hp < 0 {
            continue;
        }

        let (unactive, unactive_name, _) = q_combat_stats.get(wants_to_melee.target).unwrap();
        if unactive.hp < 0 {
            continue;
        }

        let damage = i32::max(0, active.power - unactive.defense);

        if damage == 0 {
            log.entries.push(format!(
                "{} is unable to hurt {}",
                active_name, unactive_name
            ))
        } else {
            log.entries.push(format!(
                "{} hits {}, for {} hp.",
                &active_name, &unactive_name, damage
            ));

            if let Some(tmp_damages) = damage_map.get_mut(&wants_to_melee.target) {
                tmp_damages.push(damage)
            } else {
                damage_map.insert(wants_to_melee.target, vec![damage]);
            }
        }

        commands.entity(entity).despawn_recursive();
    }

    for (entity, damages) in damage_map.into_iter() {
        let (_, _, suffer_damage) = q_combat_stats.get_mut(entity).unwrap();

        if let Some(mut suffer_damage) = suffer_damage {
            suffer_damage.amount.extend_from_slice(&damages);
        } else {
            commands
                .entity(entity)
                .insert(SufferDamage { amount: damages });
        }
    }
}

fn update_visibility(
    mut q_position: Query<(&mut Visibility, &Position, &mut Sprite, Entity)>,
    q_tiles: Query<&MapTile>,
    map: Res<Map>,
    theme: Res<Theme>,
) {
    for (mut visibility, pos, mut sprite, entity) in q_position.iter_mut() {
        let idx = map.xy_idx(pos.x, pos.y);

        if map.visible_tiles[idx] {
            *visibility = Visibility::Visible;

            if q_tiles.get(entity).is_ok() {
                let tile = map.tiles[idx];
                let glyph = theme.tile_to_render(tile);
                sprite.color = glyph.color;
            }
        } else {
            if q_tiles.get(entity).is_ok() && map.revealed_tiles[idx] {
                *visibility = Visibility::Visible;

                let tile = map.tiles[idx];

                let glyph = theme.revealed_tile_to_render(tile);
                sprite.color = glyph.color;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn update_viewshed(
    mut q_viewshed: Query<(&Position, &mut Viewshed, Entity)>,
    mut map: ResMut<Map>,
    player_entity: Res<PlayerEntity>,
) {
    for (pos, mut viewshed, entity) in q_viewshed.iter_mut() {
        if !viewshed.dirty {
            continue;
        }

        viewshed.visible_tiles.clear();
        viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
        viewshed
            .visible_tiles
            .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

        if entity == player_entity.0 {
            for t in map.visible_tiles.iter_mut() {
                *t = false
            }

            for point in viewshed.visible_tiles.iter() {
                let idx = map.xy_idx(point.x, point.y);

                map.revealed_tiles[idx] = true;
                map.visible_tiles[idx] = true;
            }
        }
    }
}

fn keep_position(mut q_position: Query<(&Position, &mut Transform), Or<(Changed<Position>,)>>) {
    for (pos, mut tran) in q_position.iter_mut() {
        let pos = Vec2::new(
            (pos.x * SPRITE_SIZE[0] as i32) as f32,
            (pos.y * SPRITE_SIZE[1] as i32) as f32,
        );

        tran.translation = pos.extend(tran.translation.z);
    }
}

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Position>();
        app.register_type::<WantsToMelee>();
        app.register_type::<SufferDamage>();
        app.insert_resource(RandomNumberGenerator(BracketRandomNumberGenerator::new()));

        app.add_systems(
            Update,
            (
                keep_position,
                update_viewshed,
                update_visibility,
                melee_combat,
                apply_damage,
                delete_the_dead,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}
