use amethyst::{
    assets::{
        AssetStorage,
        Loader
    },
    core::{
        nalgebra::Vector2,
        transform::Transform,
        Time
    },
    input,
    prelude::*,
    renderer::{
        PngFormat, 
        SpriteRender, 
        SpriteSheet, 
        SpriteSheetFormat, 
        SpriteSheetHandle, 
        Texture, 
        TextureMetadata, 
        Camera,
        Projection,
        VirtualKeyCode
    },
    ecs::{
        Dispatcher,
        DispatcherBuilder,
        Write,
        Entity
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
        }
    },
    systems,
    states::{
        PauseState
    }
};

pub struct GameplayState {
    dispatcher: Option<Dispatcher<'static, 'static>>,
    entities: Vec<Entity>,
    paused: bool
}

impl GameplayState {
    
    pub fn new() -> Self {
        return GameplayState {
            dispatcher: None,
            entities: Vec::new(),
            paused: false
        };
    }

}

impl GameplayState {

    fn initialize_dispatcher(&mut self, world: &mut World) {
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
        self.entities.drain(..).for_each(|entity| {
            world.delete_entity(entity).expect("Failed to delete entity");
        });
    }
    
    // fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    //     let texture_handle = {
    //         let loader = world.read_resource::<Loader>();
    //         let texture_storage = world.read_resource::<AssetStorage<Texture>>();

    //         loader.load(
    //             "sprites/sheet.png",
    //             PngFormat,
    //             TextureMetadata::srgb_scale(),
    //             (),
    //             &texture_storage
    //         )
    //     };

    //     let loader = world.read_resource::<Loader>();
    //     let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

    //     return loader.load(
    //         "sprites/sheet.ron",
    //         SpriteSheetFormat,
    //         texture_handle,
    //         (),
    //         &sprite_sheet_store
    //     );
    // }

    fn initialise_player_ship(world: &mut World/*, sprite_sheet: SpriteSheetHandle*/) {
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

}

impl SimpleState for GameplayState {

	fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        self.initialize_dispatcher(world);
        
        // let sprite_sheet_handle = world.read_resource::<SpriteSheetHandle>();
    
        GameplayState::initialise_player_ship(world/*, sprite_sheet_handle.clone()*/);
        GameplayState::initialise_camera(world);
        // world.add_resource(sprite_sheet_handle);
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
        // data.data.update(&mut data.world);

        if !self.paused {
            self.dispatcher.as_mut().unwrap().dispatch(&data.world.res);
        }

        return Trans::None;
    }

}
