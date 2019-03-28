use amethyst::{
    ecs::Entity,
    prelude::*,
    ui::{
        Anchor,
        UiButtonBuilder,
        UiEventType,
        UiText,
        UiTransform
    }
};
use crate::{
    constants,
    resources::UiAssets,
    states::GameplayState
};

pub struct MainMenuState {
    ui_elements: Vec<Entity>
}

impl MainMenuState {

    pub fn new() -> Self {
        return Self {
            ui_elements: Vec::new()
        }
    }

    fn create_menu(&mut self, world: &mut World) {
        let (font, button_image, button_hover_image) = {
            let ui_assets = world.read_resource::<UiAssets>();
            
            (
                ui_assets.get_font(),
                ui_assets.get_btn_img(),
                ui_assets.get_btn_hover_img()
            )
        };

        let x = 0.0;
        let title_text = UiText::new(
            font.clone(),
            String::from("Space Shooter"),
            constants::UI_FONT_COLOR,
            48.0
        );
        let title_text_transform = UiTransform::new(
            String::from("title_txt"),
            Anchor::TopMiddle,
            x,
            -100.0,
            1.0,
            400.0,
            100.0,
            1
        );
        let title_text = world
            .create_entity()
            .with(title_text)
            .with(title_text_transform)
            .build();
        self.ui_elements.push(title_text);

        let y = constants::UI_BUTTON_HEIGHT + constants::UI_BUTTON_HEIGHT / 4.0;

        let start_button = UiButtonBuilder::new("start_game_btn", "Start Game")
            .with_position(x, y)
            .with_size(constants::UI_BUTTON_WIDTH, constants::UI_BUTTON_HEIGHT)
            .with_anchor(Anchor::Middle)
            .with_font(font.clone())
            .with_text_color(constants::UI_FONT_COLOR)
            .with_font_size(constants::UI_BUTTON_FONT_SIZE)
            .with_image(button_image.clone())
            .with_hover_image(button_hover_image.clone())
            .build_from_world(world);
        self.ui_elements.push(start_button);
        
        let quit_button = UiButtonBuilder::new("quit_game_btn", "Quit")
            .with_position(x, y - constants::UI_BUTTON_HEIGHT - constants::UI_BUTTON_HEIGHT / 4.0)
            .with_size(constants::UI_BUTTON_WIDTH, constants::UI_BUTTON_HEIGHT)
            .with_anchor(Anchor::Middle)
            .with_font(font.clone())
            .with_text_color(constants::UI_FONT_COLOR)
            .with_font_size(constants::UI_BUTTON_FONT_SIZE)
            .with_image(button_image.clone())
            .with_hover_image(button_hover_image.clone())
            .build_from_world(world);
        self.ui_elements.push(quit_button);
    }

    fn clear_menu(&mut self, world: &mut World) {
        self.ui_elements.drain(..).for_each(|button| {
            world
                .delete_entity(button)
                .expect("Failed to delete button");
        })
    }

}

impl SimpleState for MainMenuState {

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        return match &event {
            StateEvent::Ui(ui_event) => {
                match ui_event.event_type {
                    UiEventType::Click => {
                        if let Some(ui_transform) = data.world.read_storage::<UiTransform>().get(ui_event.target) {
                            match ui_transform.id.as_ref() {
                                "start_game_btn" => Trans::Push(Box::new(GameplayState::new())),
                                "quit_game_btn" => Trans::Pop,
                                _ => Trans::None
                            }
                        } else {
                            Trans::None
                        }
                    },
                    _ => Trans::None
                }
            },
            _ => Trans::None
        }
    }

    fn on_start(&mut self, data: StateData<GameData>) {
        self.create_menu(data.world);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        self.clear_menu(data.world);
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        self.create_menu(data.world);
    }

    fn on_pause(&mut self, data: StateData<GameData>) {
        self.clear_menu(data.world);
    }

}