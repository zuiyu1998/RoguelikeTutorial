#![allow(clippy::type_complexity)]

mod audio;
mod common;
mod consts;
mod core;
mod enemy;
mod item;
mod loading;
mod logic;
mod map;
mod menu;
mod player;
mod render;
mod spawner;
mod state;
mod theme;
mod ui;

#[cfg(feature = "dev")]
mod dev;

pub use state::{AppState, GameState};

use crate::audio::InternalAudioPlugin;
use crate::common::CommonPlugin;
use crate::core::InternalCorePlugin;
use crate::enemy::EnemyPlugin;
use crate::item::ItemPlugin;
use crate::loading::LoadingPlugin;
use crate::logic::LogicPlugin;
use crate::map::MapPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;
use crate::state::StatePlugin;
use crate::theme::ThemePlugin;
use crate::ui::InternalUiPlugin;
use seldom_state::StateMachinePlugin;

use bevy::app::App;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LoadingPlugin,
            MenuPlugin,
            InternalAudioPlugin,
            LogicPlugin,
            CommonPlugin,
            PlayerPlugin,
            MapPlugin,
            ThemePlugin,
            EnemyPlugin,
            InternalUiPlugin,
            ItemPlugin,
            StateMachinePlugin,
            StatePlugin,
            InternalCorePlugin,
        ));

        #[cfg(feature = "dev")]
        {
            use dev::DevPlugin;

            app.add_plugins(DevPlugin);
        }
    }
}
