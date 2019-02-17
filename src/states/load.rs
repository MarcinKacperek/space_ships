use amethyst::{
    assets::{
        AssetStorage,
        Handle,
        Loader
    },
    prelude::*,
    renderer::{
        PngFormat,
        SpriteSheet,
        SpriteSheetFormat,
        Texture,
        TextureMetadata
    },
    ui::{
        FontAsset,
        TtfFormat
    }
};
use crate::components::data::UiAssets;

pub struct LoadingState {
    load_complete: bool
}

impl LoadingState {

    fn new() -> Self {
        return LoadingState {
            load_complete: false
        };
    }

    fn load_sprite_sheet(world: &mut World) {
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

    fn load_ui_assets(world: &mut World) {
        let font = LoadingState::load_font(world);
        LoadingState::show_loading_view(world);
        let button_img = LoadingState::load_btn_img(world);
        let button_hover_img = LoadingState::load_btn_hover_img(world);

        let ui_assets = UiAssets::new(
            font,
            button_img,
            button_hover_img
        );
        world.add_resource(ui_assets);
    }

    fn load_font(world: &mut World) -> Handle<FontAsset> {
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

    fn load_btn_img(world: &mut World) -> Handle<Texture> {
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

    fn load_btn_hover_img(world: &mut World) -> Handle<Texture> {
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

    fn show_loading_view(world: &mut World) {

    }

}

impl SimpleState for LoadingState {

    fn on_start(&mut self, data: StateData<GameData>) {
        LoadingState::load_ui_assets(data.world);

    }

    fn update(&mut self, _data: &mut StateData<GameData>) -> SimpleTrans {
        if self.load_complete {
            // TODO Trans::Switch Main Menu
        }

        Trans::None
    }

}