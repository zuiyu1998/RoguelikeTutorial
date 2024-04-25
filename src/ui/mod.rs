mod hub;
mod tooltip;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use hub::HudPlugin;

use self::tooltip::TooltipsPlugin;

#[derive(Component)]
pub struct TopUINode;

#[derive(Resource, AssetCollection)]
pub struct FontManager {
    #[asset(path = "fonts/VonwaonBitmap-16pxLite.ttf")]
    pub font: Handle<Font>,
}

pub struct InternalUiPlugin;

impl Plugin for InternalUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((HudPlugin, TooltipsPlugin));
    }
}
