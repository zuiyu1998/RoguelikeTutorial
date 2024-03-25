#![allow(clippy::type_complexity)]

mod audio;
mod common;
mod loading;
mod menu;
mod render;

mod logic;

#[cfg(feature = "dev")]
mod dev;

use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::logic::LogicPlugin;
use crate::menu::MenuPlugin;
use crate::render::InternalRenderPlugin;

use bevy::app::App;

use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            LoadingPlugin,
            MenuPlugin,
            InternalAudioPlugin,
            InternalRenderPlugin,
            LogicPlugin,
        ));

        #[cfg(feature = "dev")]
        {
            use dev::DevPlugin;

            app.add_plugins(DevPlugin);
        }
    }
}
