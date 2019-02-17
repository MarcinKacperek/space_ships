use amethyst::{
    core::{
        Transform,
        Time,
        nalgebra::Vector2
    },
    ecs::{
        Join,
        Entities,
        ReadStorage,
        WriteStorage,
        ReadExpect,
        Read,
        System
    },
    renderer::{
        SpriteRender, 
        SpriteSheetHandle
    }
};
use crate::{
    constants,
    components::{
        SpaceShip,
        Missile,
        Moveable,
        Rect,
        tags::{
            PlayerShipTag,
            DestroyOutOfArenaTag
        }
    }
};

pub struct ShootingSystem;

impl<'s> System<'s> for ShootingSystem {
    type SystemData = (
        WriteStorage<'s, SpaceShip>,
        ReadStorage<'s, PlayerShipTag>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Missile>,
        WriteStorage<'s, Moveable>,
        WriteStorage<'s, Rect>,
        WriteStorage<'s, DestroyOutOfArenaTag>,
        ReadExpect<'s, SpriteSheetHandle>,
        Entities<'s>,
        Read<'s, Time>
    );

    fn run(
        &mut self, 
        (
            mut space_ships, 
            player_ship_tags, 
            mut transforms, 
            mut sprite_renders, 
            mut missiles, 
            mut moveables,
            mut rects,
            mut destroy_out_of_arena_tags,
            sprite_sheet_handle, 
            entities,
            time
        ): Self::SystemData
    ) {
        for (space_ship, entity) in (&mut space_ships, &entities).join() {
            if space_ship.is_attacking && space_ship.last_attack_time + space_ship.attack_cooldown <= time.absolute_time_seconds() {
                let transform = transforms.get(entity).unwrap().clone();
                let sprite_render = SpriteRender {
                    sprite_sheet: sprite_sheet_handle.clone(),
                    sprite_number: 5
                };
                let is_player = player_ship_tags.contains(entity);
                let direction_y = if is_player {
                    1.0
                } else {
                    -1.0
                };

                entities
                    .build_entity()
                    .with(transform, &mut transforms)
                    .with(sprite_render, &mut sprite_renders)
                    .with(Missile::new(1, is_player), &mut missiles)
                    .with(
                        Moveable {
                            move_speed: 500.0,
                            direction: Vector2::new(0.0, direction_y)
                        }, 
                        &mut moveables
                    )
                    .with(
                        Rect {
                            width: constants::MISSILE_WIDTH,
                            height: constants::MISSILE_HEIGHT
                        },
                        &mut rects
                    )
                    .with(DestroyOutOfArenaTag, &mut destroy_out_of_arena_tags)
                    .build();

                space_ship.last_attack_time = time.absolute_time_seconds();
            }
        }        
    }
}