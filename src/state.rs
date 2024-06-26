use bevy::{ecs::system::SystemParam, prelude::*};

#[derive(SystemParam)]
pub struct AppStateManager<'w> {
    app_next_state: ResMut<'w, NextState<AppState>>,
    game_next_state: ResMut<'w, NextState<GameState>>,
}

impl<'w> AppStateManager<'w> {
    pub fn start_game(&mut self) {
        self.app_next_state.set(AppState::InGame);
        self.game_next_state.set(GameState::Playing);
    }

    pub fn end_game(&mut self) {
        self.app_next_state.set(AppState::Menu);
        self.game_next_state.set(GameState::None);
    }

    pub fn start_tootip(&mut self) {
        self.game_next_state.set(GameState::ToolTip);
    }

    pub fn start_playing(&mut self) {
        self.game_next_state.set(GameState::Playing);
    }

    pub fn start_tab(&mut self) {
        self.game_next_state.set(GameState::Tab);
    }
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.init_state::<AppState>();
    }
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    InGame,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    None,
    Playing,
    Pause,
    //查看敌人信息
    ToolTip,
    //查看玩家信息
    Tab,
}
