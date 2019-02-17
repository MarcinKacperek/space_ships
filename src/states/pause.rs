use amethyst::{
    assets::{
        AssetStorage,
        Loader
    },
    ecs::{
        Entity
    },
    prelude::*,
    renderer::{
        PngFormat, 
        Texture, 
        TextureMetadata, 
        VirtualKeyCode
    },
    input,
    ui::{
        TtfFormat,
        UiTransform,
        UiButtonBuilder,
        UiEventType,
        Anchor
    },
    shrev::EventChannel
};
use crate::constants;

pub struct PauseState {
    buttons: Vec<Entity>
}

impl PauseState {

    pub fn new() -> Self {
        return PauseState {
            buttons: Vec::new()
        };
    }

    fn create_menu(&mut self, world: &mut World) {
        let font = world
            .read_resource::<Loader>()
            .load(
                "ui/Recharge.ttf",
                TtfFormat,
                Default::default(),
                (),
                &world.read_resource()
            );

        let image = world
            .read_resource::<Loader>()
            .load(
                "ui/button.png",
                PngFormat,
                TextureMetadata::srgb_scale(),
                (),
                &world.read_resource::<AssetStorage<Texture>>()
            );
        
        let x = 0.0;//constants::ARENA_WIDTH / 2.0 - constants::UI_BUTTON_WIDTH / 2.0;
        let y = constants::UI_BUTTON_HEIGHT + constants::UI_BUTTON_HEIGHT / 4.0;//constants::ARENA_HEIGHT / 2.0 + constants::UI_BUTTON_HEIGHT / 4.0;

        let resume_button = UiButtonBuilder::new("resume_btn", "Resume")
            .with_position(x, y)
            .with_size(constants::UI_BUTTON_WIDTH, constants::UI_BUTTON_HEIGHT)
            .with_anchor(Anchor::Middle)
            .with_font(font.clone())
            .with_text_color([0.95, 0.95, 0.95, 1.0])
            .with_font_size(constants::UI_BUTTON_FONT_SIZE)
            .with_image(image.clone())
            .build_from_world(world);
        self.buttons.push(resume_button);

        let main_menu_button = UiButtonBuilder::new("main_menu_btn", "Main Menu")
            .with_position(x, y - constants::UI_BUTTON_HEIGHT - constants::UI_BUTTON_HEIGHT / 4.0)
            .with_size(constants::UI_BUTTON_WIDTH, constants::UI_BUTTON_HEIGHT)
            .with_anchor(Anchor::Middle)
            .with_font(font.clone())
            .with_text_color([0.95, 0.95, 0.95, 1.0])
            .with_font_size(constants::UI_BUTTON_FONT_SIZE)
            .with_image(image.clone())
            .build_from_world(world);
        self.buttons.push(main_menu_button);
    }

    fn clear_menu(&mut self, world: &mut World) {
        self.buttons.drain(..).for_each(|button| {
            world
                .delete_entity(button)
                .expect("Failed to delete button");
        });
    }

}

impl SimpleState for PauseState {

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if input::is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Pop;
            }
        }

        return match &event {
            StateEvent::Ui(ui_event) => {
                match ui_event.event_type {
                    UiEventType::Click => {
                        if let Some(ui_transform) = data.world.read_storage::<UiTransform>().get(ui_event.target) {
                            match ui_transform.id.as_ref() {
                                "resume_btn" => Trans::Pop,
                                "main_menu_btn" => {
                                    data.world
                                        .write_resource::<EventChannel<TransEvent<GameData, StateEvent>>>()
                                        .single_write(Box::new(|| Trans::Pop));
                                    Trans::Pop
                                },
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

}