use amethyst::{
    assets::{
        Handle
    },
    ecs::{
        Component,
        DenseVecStorage
    },
    renderer::{
        Texture
    },
    ui::FontAsset
};

pub struct UiAssets {
    font: Handle<FontAsset>,
    btn_img: Handle<Texture>,
    btn_hover_img: Handle<Texture>
}

impl UiAssets {

    pub fn new(
        font: Handle<FontAsset>,
        btn_img: Handle<Texture>,
        btn_hover_img: Handle<Texture>
    ) -> Self {
        return UiAssets {
            font,
            btn_img,
            btn_hover_img
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

}

impl Component for UiAssets {
    type Storage = DenseVecStorage<Self>;
}