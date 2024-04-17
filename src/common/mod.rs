use crate::consts::SPRITE_SIZE;
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
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

        app.add_systems(PreUpdate, (keep_position,));
    }
}
