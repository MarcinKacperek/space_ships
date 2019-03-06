use amethyst::{
    assets::{
        Handle
    },
    ecs::{
        Component,
        DenseVecStorage,
        Entity
    },
    renderer::{
        Texture
    },
    ui::FontAsset
};

pub struct UiAssets {
    font: Handle<FontAsset>,
    btn_img: Handle<Texture>,
    btn_hover_img: Handle<Texture>,
    life_img: Handle<Texture>
}

impl UiAssets {

    pub fn new(
        font: Handle<FontAsset>,
        btn_img: Handle<Texture>,
        btn_hover_img: Handle<Texture>,
        life_img: Handle<Texture>
    ) -> Self {
        return UiAssets {
            font,
            btn_img,
            btn_hover_img,
            life_img
        };
    }

    pub fn get_font(&self) -> Handle<FontAsset> {
        return self.font.clone();
    }

    pub fn get_btn_img(&self) -> Handle<Texture> {
        return self.btn_img.clone();
    }

    pub fn get_btn_hover_img(&self) -> Handle<Texture> {
        return self.btn_hover_img.clone();
    }

    pub fn get_life_img(&self) -> Handle<Texture> {
        return self.life_img.clone();
    }

}

impl Component for UiAssets {
    type Storage = DenseVecStorage<Self>;
}

pub struct UiGameplayElements {
    pub score_value_text: Entity,
    pub life_value_text: Entity
}

impl UiGameplayElements {

    pub fn new(score_value_text: Entity, life_value_text: Entity) -> Self {
        return Self {
            score_value_text,
            life_value_text
        };
    }

}

impl Component for UiGameplayElements {
    type Storage = DenseVecStorage<Self>;
}

pub struct GameplaySessionData {
    pub score: i32
}

impl Component for GameplaySessionData {
    type Storage = DenseVecStorage<Self>;
}