use amethyst::{
    core::{
        nalgebra::Vector2,
        Parent,
        transform::Transform,
        Time
    },
    ecs::{
        Dispatcher,
        DispatcherBuilder
    },
    input,
    prelude::*,
    renderer::{
        SpriteRender, 
        SpriteSheetHandle, 
        Camera,
        Projection,
        VirtualKeyCode
    },
    ui::{
        Anchor,
        UiImage,
        UiText,
        UiTransform
    }
};
use crate::{
    constants,
    components::{
        Cannon,
        Moveable,
        Rect,
        Killable,
        SpaceShip,
        tags::{
            BoundInArenaTag,
            PlayerShipTag
        }
    },
    resources::{
        GameplayNextState,
        GameplaySessionData,
        GameState,
        UiAssets,
        UiGameplayElements
    },
    systems,
    states::{
        PauseState,
        ResultState
    }
};

pub struct GameplayState {
    dispatcher: Option<Dispatcher<'static, 'static>>,
    current_state: GameState
}

impl GameplayState {
    
    pub fn new() -> Self {
        return GameplayState {
            dispatcher: None,
            current_state: GameState::Running
        };
    }

}

impl GameplayState {

    fn initialise_dispatcher(&mut self, world: &mut World) {
        let mut dispatcher_builder = DispatcherBuilder::new();

        dispatcher_builder.add(systems::PlayerShipSystem, "player_ship_system", &[]);
        dispatcher_builder.add(systems::MovementSystem, "movement_system", &["player_ship_system"]);
        dispatcher_builder.add(systems::ShootingSystem, "shooting_system", &["player_ship_system"]);
        dispatcher_builder.add(systems::BoundInArenaSystem, "bound_in_arena_system", &["movement_system"]);
        dispatcher_builder.add(systems::DestroyOutOfArenaSystem, "destroy_out_of_arena_system", &["bound_in_arena_system"]);
        dispatcher_builder.add(systems::MissileSystem, "missile_system", &["movement_system", "shooting_system"]);
        dispatcher_builder.add(systems::KillSystem, "kill_system", &["missile_system"]);
        dispatcher_builder.add(systems::EnemyCollisionSystem, "enemy_collision", &["kill_system"]);
        dispatcher_builder.add(systems::EnemySpawnerSystem::default(), "enemy_spawner", &["destroy_out_of_arena_system"]);
        // dispatcher_builder.add(systems::ClearCannonsSystem, "clear_cannons", &["enemy_collision", "destroy_out_of_arena_system"]);
        dispatcher_builder.add(systems::ClearChildrenSystem, "clear_children", &["enemy_collision", "destroy_out_of_arena_system"]);
        dispatcher_builder.add(systems::DeleteEntitiesSystem, "delete_entities", &["clear_children"]);
        dispatcher_builder.add(systems::UiSystem, "ui", &["enemy_collision"]);

        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(&mut world.res);
        self.dispatcher = Some(dispatcher);
    }

    fn terminate_dispatcher(&mut self) {
        self.dispatcher = None;
    }

    fn terminate_entities(&mut self, world: &mut World) {
        world.delete_all();
    }

    fn initialise_player_ship(world: &mut World) {
        let mut transform: Transform = Transform::default();

        let player_scale = 0.4;
        let player_width = 99.0 * player_scale;
        let player_height = 75.0 * player_scale;

        let missile_x_offset = 0.0;
        let missile_y_offset = player_height / 2.0;
        let missile_width = 9.0 * player_scale;
        let missile_height = 37.0 * player_scale;

        let x = constants::ARENA_WIDTH / 2.0;
        let y = player_height / 2.0;
        transform.set_xyz(x, y, 0.0);
        transform.set_scale(player_scale, player_scale, 1.0);

        let sprite_render = {
            let sprite_sheet = world.read_resource::<SpriteSheetHandle>();

            SpriteRender {
                sprite_sheet: sprite_sheet.clone(),
                sprite_number: 0
            }
        };

        let player_entity = world
            .create_entity()
            .with(sprite_render)
            .with(transform)
            .with(PlayerShipTag)
            .with(BoundInArenaTag)
            .with(Rect {
                width: player_width,
                height: player_height
            })
            .with(Moveable {
                move_speed: 250.0,
                direction: Vector2::new(0.0, 0.0)
            })
            .with(Killable::new(3))
            .with(SpaceShip {
                is_attacking: false,
                cannon_entities_indices: Vec::new()
            })
            .build();
        let cannon_entity = world
            .create_entity()
            .with(Parent {
                entity: player_entity
            })
            .with(Cannon {
                x_offset: missile_x_offset,
                y_offset: missile_y_offset,
                attack_cooldown: 0.5,
                last_attack_time: 0.0,
                missile_width: missile_width,
                missile_height: missile_height,
                missile_speed: 500.0,
                missile_sprite_index: 13
            })
            .build();

        let mut space_ship_storage = world.write_storage::<SpaceShip>();
        let player_space_ship = space_ship_storage.get_mut(player_entity).unwrap();
        player_space_ship.cannon_entities_indices.push(cannon_entity.id());
    }

    fn initialise_camera(world: &mut World) {
        let mut transform: Transform = Transform::default();
        transform.set_z(1.0);
        world
            .create_entity()
            .with(Camera::from(Projection::orthographic(
                0.0, 
                constants::ARENA_WIDTH,
                0.0,
                constants::ARENA_HEIGHT
            )))
            .with(transform)
            .build();
    }

    fn initialise_gameplay_session_data(world: &mut World) {
        let session_data = GameplaySessionData{ score: 0 };
        world.add_resource(session_data);
    }

    fn initialise_ui(world: &mut World) {
        let (font, life_img) = {
            let ui_assets = world.read_resource::<UiAssets>();
            (ui_assets.get_font(), ui_assets.get_life_img())
        };

        // Initialise score
        let score_text = UiText::new(
            font.clone(),
            String::from("Score:"),
            constants::UI_FONT_COLOR,
            constants::UI_GAMEPLAY_FONT_SIZE
        );
        let score_text_transform = UiTransform::new(
            String::from("score_txt"),
            Anchor::TopRight,
            -100.0,
            -constants::UI_GAMEPLAY_FONT_SIZE,
            1.0,
            80.0,
            constants::UI_GAMEPLAY_FONT_SIZE,
            1
        );
        world
            .create_entity()
            .with(score_text)
            .with(score_text_transform)
            .build();

        let score_value_text = UiText::new(
            font.clone(),
            String::from("0"),
            constants::UI_FONT_COLOR,
            constants::UI_GAMEPLAY_FONT_SIZE
        );
        let score_value_text_transform = UiTransform::new(
            String::from("score_value_txt"),
            Anchor::TopRight,
            -40.0,
            -constants::UI_GAMEPLAY_FONT_SIZE,
            1.0,
            40.0,
            constants::UI_GAMEPLAY_FONT_SIZE,
            1
        );
        let score_value_text = world
            .create_entity()
            .with(score_value_text)
            .with(score_value_text_transform)
            .build();
        // Initialise Life
        let life_image = UiImage {
            texture: life_img
        };
        let life_image_transform = UiTransform::new(
            String::from("life_img"),
            Anchor::BottomLeft,
            25.0,
            25.0,
            1.0,
            16.0,
            16.0,
            1
        );
        world
            .create_entity()
            .with(life_image)
            .with(life_image_transform)
            .build();

        let life_text = UiText::new(
            font.clone(),
            String::from("x"),
            constants::UI_FONT_COLOR,
            constants::UI_GAMEPLAY_FONT_SIZE
        );
        let life_text_transform = UiTransform::new(
            String::from("life_txt"),
            Anchor::BottomLeft,
            41.0,
            25.0,
            1.0,
            8.0,
            8.0,
            1
        );
        world
            .create_entity()
            .with(life_text)
            .with(life_text_transform)
            .build();
        
        let life_value_text = UiText::new(
            font.clone(),
            constants::PLAYER_LIVES.to_string(),
            constants::UI_FONT_COLOR,
            constants::UI_GAMEPLAY_FONT_SIZE
        );
        let life_value_text_transform = UiTransform::new(
            String::from("life_value_txt"),
            Anchor::BottomLeft,
            55.0,
            25.0,
            1.0,
            16.0,
            16.0,
            1
        );
        let life_value_text = world
            .create_entity()
            .with(life_value_text)
            .with(life_value_text_transform)
            .build();

        world.add_resource(UiGameplayElements::new(score_value_text, life_value_text));
    }

    fn initialise_gameplay_resources(world: &mut World) {
        world.add_resource(GameplayNextState { next_state: None });
    }

}

impl SimpleState for GameplayState {

	fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        self.initialise_dispatcher(world);
        
        GameplayState::initialise_player_ship(world);
        GameplayState::initialise_camera(world);
        GameplayState::initialise_gameplay_session_data(world);
        GameplayState::initialise_ui(world);
        GameplayState::initialise_gameplay_resources(world);
    }

    fn on_stop(&mut self, mut data: StateData<GameData>) {
        self.terminate_entities(&mut data.world);
        self.terminate_dispatcher();
    }

    fn on_pause(&mut self, data: StateData<GameData>) {
        let world = data.world;

        let mut gameplay_next_state = world.write_resource::<GameplayNextState>();
        gameplay_next_state.next_state = Some(GameState::Paused);

        let mut time = world.write_resource::<Time>();
        time.set_time_scale(0.0);
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        let world = data.world;

        let mut gameplay_next_state = world.write_resource::<GameplayNextState>();
        gameplay_next_state.next_state = Some(GameState::Running);

        let mut time = world.write_resource::<Time>();
        time.set_time_scale(1.0);
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if input::is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(PauseState::new()));
            }
        }

        return Trans::None;
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        {
            // Change state
            let mut gameplay_next_state = data.world.write_resource::<GameplayNextState>();
            if let Some(next_state) = &gameplay_next_state.next_state {
                self.current_state = next_state.clone();
                gameplay_next_state.next_state = None;
            }
        }
        
        match self.current_state {
            GameState::Running => self.dispatcher.as_mut().unwrap().dispatch(&data.world.res),
            GameState::Paused => {
                // Do nothing
            },
            GameState::Finished => {
                return Trans::Push(Box::new(ResultState::new()));
            }
        };

        return Trans::None;
    }

}
