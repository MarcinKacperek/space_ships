use amethyst::{
    core::{
        nalgebra::Vector2,
        transform::Transform,
        Time
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
    ecs::{
        Dispatcher,
        DispatcherBuilder,
        Write
    },
    ui::{
        UiText
    }
};
use crate::{
    constants,
    components::{
        Moveable,
        Rect,
        Killable,
        SpaceShip,
        tags::{
            PlayerShipTag,
            BoundInArenaTag
        },
        data::{
            GameplaySessionData,
            UiAssets
        }
    },
    systems,
    states::{
        PauseState
    }
};

pub struct GameplayState {
    dispatcher: Option<Dispatcher<'static, 'static>>,
    paused: bool
}

impl GameplayState {
    
    pub fn new() -> Self {
        return GameplayState {
            dispatcher: None,
            paused: false
        };
    }

}

impl GameplayState {

    fn initialise_dispatcher(&mut self, world: &mut World) {
        let mut dispatcher_builder = DispatcherBuilder::new();

        dispatcher_builder.add(systems::PlayerShipSystem, "player_ship_system", &[]);
        dispatcher_builder.add(systems::MovementSystem, "movement_system", &["player_ship_system"]);
        dispatcher_builder.add(systems::ShootingSystem, "shooting_system", &["player_ship_system"]);
        dispatcher_builder.add(systems::MissileSystem, "missile_system", &["movement_system", "shooting_system"]);
        dispatcher_builder.add(systems::BoundInArenaSystem, "bound_in_arena_system", &["movement_system"]);
        dispatcher_builder.add(systems::DestroyOutOfArenaSystem, "destroy_out_of_arena_system", &["bound_in_arena_system"]);
        dispatcher_builder.add(systems::KillSystem, "kill_system", &["missile_system"]);
        dispatcher_builder.add(systems::EnemySpawnerSystem::default(), "enemy_spawner", &["destroy_out_of_arena_system"]);

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

        let x = constants::ARENA_WIDTH / 2.0;
        let y = constants::PLAYER_SHIP_HEIGHT / 2.0;
        transform.set_xyz(x, y, 0.0);

        let sprite_render = {
            let sprite_sheet = world.read_resource::<SpriteSheetHandle>();

            SpriteRender {
                sprite_sheet: sprite_sheet.clone(),
                sprite_number: 0
            }
        };

        world
            .create_entity()
            .with(sprite_render)
            .with(transform)
            .with(PlayerShipTag)
            .with(BoundInArenaTag)
            .with(Rect {
                width: constants::PLAYER_SHIP_WIDTH,
                height: constants::PLAYER_SHIP_HEIGHT
            })
            .with(Moveable {
                move_speed: 250.0,
                direction: Vector2::new(0.0, 0.0)
            })
            .with(Killable::new(3))
            .with(SpaceShip {
                attack_cooldown: 0.5,
                last_attack_time: 0.0,
                is_attacking: false
            })
            .build();
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
        let ui_assets = world.read_resource::<UiAssets>();
        // Initialise score
        let score_text = UiText::new(
            ui_assets.get_font(),
            String::from("Score:"),
            [0.95, 0.95, 0.95, 1.0],
            constants::UI_GAMEPLAY_FONT_SIZE
        );
        
        // Initialise health

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
    }

    fn on_stop(&mut self, mut data: StateData<GameData>) {
        self.terminate_entities(&mut data.world);
        self.terminate_dispatcher();
    }

    fn on_pause(&mut self, data: StateData<GameData>) {
        self.paused = true;
        data.world.exec(|mut time: Write<Time>| {
            time.set_time_scale(0.0);
        });
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        self.paused = false;
        data.world.exec(|mut time: Write<Time>| {
            time.set_time_scale(1.0);
        });
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
        if !self.paused {
            self.dispatcher.as_mut().unwrap().dispatch(&data.world.res);
        }

        return Trans::None;
    }

}
