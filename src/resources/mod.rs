use amethyst::{
    assets::{
        Handle
    },
    ecs::Entity,
    renderer::{
        Texture
    },
    ui::FontAsset
};

#[derive(Clone)]
pub enum GameState {
    Running,
    Paused,
    Finished
}

pub struct GameplayNextState {
    pub next_state: Option<GameState>
}

pub struct UiAssets {
    font: Handle<FontAsset>,
    btn_img: Handle<Texture>,
    btn_hover_img: Handle<Texture>,
    life_img: Handle<Texture>,
    health_bar_border_img: Handle<Texture>,
    health_bar_green_img: Handle<Texture>,
    health_bar_red_img: Handle<Texture>
}

impl UiAssets {

    pub fn new(
        font: Handle<FontAsset>,
        btn_img: Handle<Texture>,
        btn_hover_img: Handle<Texture>,
        life_img: Handle<Texture>,
        health_bar_border_img: Handle<Texture>,
        health_bar_green_img: Handle<Texture>,
        health_bar_red_img: Handle<Texture>
    ) -> Self {
        return UiAssets {
            font,
            btn_img,
            btn_hover_img,
            life_img,
            health_bar_border_img,
            health_bar_green_img,
            health_bar_red_img
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

    pub fn get_health_bar_border_img(&self) -> Handle<Texture> {
        return self.health_bar_border_img.clone();
    }

    pub fn get_health_bar_green_img(&self) -> Handle<Texture> {
        return self.health_bar_green_img.clone();
    }

    pub fn get_health_bar_red_img(&self) -> Handle<Texture> {
        return self.health_bar_red_img.clone();
    }

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

pub struct GameplaySessionData {
    pub score: i32
}