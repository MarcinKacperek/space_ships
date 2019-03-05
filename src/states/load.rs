use amethyst::{
    assets::{
        AssetStorage,
        Handle,
        Loader
    },
    ecs::Entity,
    prelude::*,
    renderer::{
        PngFormat,
        SpriteSheet,
        SpriteSheetFormat,
        Texture,
        TextureMetadata
    },
    ui::{
        Anchor,
        FontAsset,
        TtfFormat,
        UiText,
        UiTransform
    }
};
use crate::{
    components::data::UiAssets,
    constants,
    states::MainMenuState
};

pub struct LoadingState {
    loading_text: Option<Entity>,
    load_complete: bool
}

impl LoadingState {

    pub fn new() -> Self {
        return LoadingState {
            loading_text: None,
            load_complete: false
        };
    }

    fn load_assets(&mut self, world: &mut World) {
        self.load_ui_assets(world);
        self.load_sprite_sheet(world);

        self.load_complete = true;
    }

    fn load_sprite_sheet(&mut self, world: &mut World) {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();

            loader.load(
                "sprites/sheet.png",
                PngFormat,
                TextureMetadata::srgb_scale(),
                (),
                &texture_storage
            )
        };

        let sprite_sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

            loader.load(
                "sprites/sheet.ron",
                SpriteSheetFormat,
                texture_handle,
                (),
                &sprite_sheet_store
            )
        };

        world.add_resource(sprite_sheet_handle);
    }

    fn load_ui_assets(&mut self, world: &mut World) {
        let font = self.load_font(world);
        self.show_loading_view(world, font.clone());
        let button_img = self.load_btn_img(world);
        let button_hover_img = self.load_btn_hover_img(world);
        let life_img = self.load_life_img(world);

        let ui_assets = UiAssets::new(
            font,
            button_img,
            button_hover_img,
            life_img
        );
        world.add_resource(ui_assets);
    }

    fn load_font(&mut self, world: &mut World) -> Handle<FontAsset> {
        return world
            .read_resource::<Loader>()
            .load(
                "ui/Recharge.ttf",
                TtfFormat,
                Default::default(),
                (),
                &world.read_resource()
            );
    }

    fn load_btn_img(&mut self, world: &mut World) -> Handle<Texture> {
        return world
            .read_resource::<Loader>()
            .load(
                "ui/button.png",
                PngFormat,
                TextureMetadata::srgb_scale(),
                (),
                &world.read_resource::<AssetStorage<Texture>>()
            );
    }

    fn load_btn_hover_img(&mut self, world: &mut World) -> Handle<Texture> {
         return world
            .read_resource::<Loader>()
            .load(
                "ui/button_hover.png",
                PngFormat,
                TextureMetadata::srgb_scale(),
                (),
                &world.read_resource::<AssetStorage<Texture>>()
            );
    }

    fn load_life_img(&mut self, world: &mut World) -> Handle<Texture> {
        return world
            .read_resource::<Loader>()
            .load(
                "ui/life.png",
                PngFormat,
                TextureMetadata::srgb_scale(),
                (),
                &world.read_resource::<AssetStorage<Texture>>()
            );
    }

    fn show_loading_view(&mut self, world: &mut World, font: Handle<FontAsset>) {
        let loading_text = UiText::new(
            font.clone(),
            String::from("Loading..."),
            constants::UI_FONT_COLOR,
            constants::UI_BUTTON_FONT_SIZE
        );

        let loading_text_transform = UiTransform::new(
            String::from("loading_txt"),
            Anchor::Middle,
            0.0,
            0.0,
            1.0,
            constants::UI_BUTTON_WIDTH,
            constants::UI_BUTTON_HEIGHT,
            1
        );

        let loading_text = world
            .create_entity()
            .with(loading_text)
            .with(loading_text_transform)
            .build();
        self.loading_text = Some(loading_text);
    }

}

impl SimpleState for LoadingState {

    fn on_start(&mut self, data: StateData<GameData>) {
        self.load_assets(data.world);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(loading_text) = self.loading_text {
            data.world.delete_entity(loading_text).expect("Failed to delete text");
        }
    }

    fn update(&mut self, _data: &mut StateData<GameData>) -> SimpleTrans {
        if self.load_complete {
            return Trans::Switch(Box::new(MainMenuState::new()));
        }

        return Trans::None;
    }

}