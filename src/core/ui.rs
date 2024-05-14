use bevy::{
    ecs::system::{SystemParam, SystemState},
    prelude::*,
};
use bevy_egui::{
    egui::{self, FontData, FontDefinitions, Ui},
    EguiContexts, EguiUserTextures,
};

use crate::AppState;

use super::TextureAssets;

pub struct CoreUiPlugin;

impl Plugin for CoreUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), configure_visuals_system);

        #[cfg(not(feature = "dev"))]
        {
            use bevy_egui::EguiPlugin;
            app.add_plugins((EguiPlugin));
        }
    }
}

fn set_fonts(context: &mut egui::Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "VonwaonBitmap".to_owned(),
        FontData::from_static(include_bytes!(
            "../../assets/fonts/VonwaonBitmap-16pxLite.ttf"
        )),
    );

    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "VonwaonBitmap".to_owned());

    context.set_fonts(fonts);
}

fn set_textures(contexts: &mut EguiContexts, texture_assets: &TextureAssets) {
    contexts.add_image(texture_assets.i.clone());
}

fn configure_visuals_system(mut contexts: EguiContexts, texture_assets: Res<TextureAssets>) {
    contexts.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        window_shadow: egui::epaint::Shadow::NONE,
        ..Default::default()
    });

    set_fonts(contexts.ctx_mut());

    set_textures(&mut contexts, &texture_assets);
}

pub struct EguiUiContext {
    context: egui::Context,
    textures: EguiUserTextures,
}

impl EguiUiContext {
    pub fn get(&self) -> &egui::Context {
        &self.context
    }
}

pub struct BevyBuildContext<T> {
    pub item: T,
}

pub struct EguiWidghtBuildContext<'a, T> {
    pub item: T,
    pub ui_context: &'a EguiUiContext,
}

impl<'a, T> EguiWidghtBuildContext<'a, T> {
    pub fn new(item: T, ui_context: &'a EguiUiContext) -> Self {
        EguiWidghtBuildContext { item, ui_context }
    }
}

impl<'a, T> EguiWidghtBuildContext<'a, T> {
    pub fn image_id(&self, image: &Handle<Image>) -> Option<egui::TextureId> {
        self.ui_context.textures.image_id(image)
    }
}

pub trait UiSystem: SystemParam + 'static {
    type UiState: UiContainer<Self>;

    fn extra_ui_state(item: &<Self as SystemParam>::Item<'_, '_>) -> Self::UiState;

    fn ui(
        ui_state: Self::UiState,
        ui_context: EguiUiContext,
        bevy_context: BevyBuildContext<<Self as SystemParam>::Item<'_, '_>>,
    ) {
        ui_state.container(ui_context, bevy_context);
    }

    fn show_ui(world: &mut World) {
        let mut state = SystemState::<EguiContexts>::new(world);
        let mut contexts = state.get_mut(world);
        let context = contexts.ctx_mut().clone();

        let textures = world.get_resource::<EguiUserTextures>().unwrap().clone();

        let mut state = SystemState::<Self>::new(world);
        let item = state.get_mut(world);

        let ui_state = Self::extra_ui_state(&item);

        let ui_context: EguiUiContext = EguiUiContext { context, textures };

        let build_context = BevyBuildContext { item };

        Self::ui(ui_state, ui_context, build_context);
    }
}

pub trait UiContainer<T>
where
    T: SystemParam + 'static,
{
    fn container(
        &self,
        ui_context: EguiUiContext,
        bevy_context: BevyBuildContext<<T as SystemParam>::Item<'_, '_>>,
    );
}

pub trait UiWidght<T> {
    fn widght<'a>(&self, context: EguiWidghtBuildContext<'a, T>, ui: &'a mut Ui);
}

pub trait BuildUiWidght<T, P>
where
    P: SystemParam + 'static,
    Self: Sized,
{
    fn build_widght(
        &self,
        ui_context: &EguiUiContext,
        bevy_context: &mut BevyBuildContext<<P as SystemParam>::Item<'_, '_>>,
        ui: &mut Ui,
    );
}
