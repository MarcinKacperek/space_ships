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
        Anchor,
        LineMode,
        FontAsset,
        TtfFormat,
        UiText,
        UiTransform
    }
};
use crate::{
    components::data::UiAssets,
    constants,
    states::GameplayState
};

pub struct LoadingState {
    load_complete: bool
}

impl LoadingState {

    pub fn new() -> Self {
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
        LoadingState::show_loading_view(world, font.clone());
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
                "ui/button_hover.png",
                PngFormat,
                TextureMetadata::srgb_scale(),
                (),
                &world.read_resource::<AssetStorage<Texture>>()
            );
    }

    fn show_loading_view(world: &mut World, font: Handle<FontAsset>) {
        let mut loading_text = UiText::new(
            font.clone(),
            String::from("Loading..."),
            [0.95, 0.95, 0.95, 1.0],
            constants::UI_BUTTON_FONT_SIZE
        );
        loading_text.line_mode = LineMode::Single;
        loading_text.align = Anchor::Middle;

        let loading_text_transform = UiTransform::new(
            String::from("loading_txt"),
            Anchor::Middle,
            constants::ARENA_WIDTH / 2.0,
            constants::ARENA_HEIGHT / 2.0,
            1.0,
            constants::UI_BUTTON_WIDTH,
            constants::UI_BUTTON_HEIGHT,
            1
        );

        world
            .create_entity()
            .with(loading_text)
            .with(loading_text_transform)
            .build();
    }

}

impl SimpleState for LoadingState {

    fn on_start(&mut self, data: StateData<GameData>) {
        LoadingState::load_ui_assets(data.world);
        LoadingState::load_sprite_sheet(data.world);

        self.load_complete = true;
    }

    fn update(&mut self, _data: &mut StateData<GameData>) -> SimpleTrans {
        if self.load_complete {
            return Trans::Switch(Box::new(GameplayState::new()));
            // TODO Trans::Switch Main Menu
        }

        return Trans::None;
    }

}