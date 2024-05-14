mod ui;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub use ui::*;

pub mod prelude {
    pub use super::ui::*;
}

pub struct InternalCorePlugin;

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
    #[asset(path = "textures/i.png")]
    pub i: Handle<Image>,
    #[asset(path = "textures/terminal8x8_transparent.png")]
    pub terminal: Handle<Image>,
}

impl Plugin for InternalCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CoreUiPlugin);
    }
}
