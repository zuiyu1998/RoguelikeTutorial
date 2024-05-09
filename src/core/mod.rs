mod ui;

use bevy::prelude::*;

pub use ui::CoreUiPlugin;

pub struct InternalCorePlugin;

impl Plugin for InternalCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CoreUiPlugin);
    }
}
