use amethyst::{
    core::{
        nalgebra::Vector2,
        Parent,
        Transform,
        Time
    },
    ecs::{
        Entities,
        Join,
        Read,
        ReadExpect,
        ReadStorage,
        System,
        WriteStorage
    },
    renderer::{
        SpriteRender, 
        SpriteSheetHandle
    }
};
use crate::{
    components::{
        Cannon,
        Missile,
        Moveable,
        Rect,
        SpaceShip,
        tags::{
            PlayerShipTag,
            DestroyOutOfArenaTag
        }
    }
};

pub struct ShootingSystem;

impl<'s> System<'s> for ShootingSystem {
    type SystemData = (
        ReadStorage<'s, SpaceShip>,
        ReadStorage<'s, PlayerShipTag>,
        ReadStorage<'s, Parent>,
        WriteStorage<'s, Cannon>,
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
            space_ships, 
            player_ship_tags, 
            parents,
            mut cannons,
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
        for (cannon, parent) in (&mut cannons, &parents).join() {
            let space_ship = space_ships.get(parent.entity).expect("Parent entity doesn't have SpaceShip component!");
            if space_ship.is_attacking && cannon.last_attack_time + cannon.attack_cooldown <= time.absolute_time_seconds() {
                let mut transform = transforms.get_mut(parent.entity).unwrap().clone();
                transform.translate_xyz(cannon.x_offset, cannon.y_offset, 0.0);
                let is_player = player_ship_tags.contains(parent.entity);
                let direction_y = if is_player {
                    1.0
                } else {
                    -1.0
                };

                entities
                    .build_entity()
                    .with(transform, &mut transforms)
                    .with(
                        SpriteRender {
                            sprite_sheet: sprite_sheet_handle.clone(),
                            sprite_number: cannon.missile_sprite_index
                        }, 
                        &mut sprite_renders
                    )
                    .with(Missile::new(is_player), &mut missiles)
                    .with(
                        Moveable {
                            move_speed: cannon.missile_speed,
                            direction: Vector2::new(0.0, direction_y)
                        }, 
                        &mut moveables
                    )
                    .with(
                        Rect {
                            width: cannon.missile_width,
                            height: cannon.missile_height
                        },
                        &mut rects
                    )
                    .with(DestroyOutOfArenaTag, &mut destroy_out_of_arena_tags)
                    .build();

                cannon.last_attack_time = time.absolute_time_seconds();
            }
        }

        // for (space_ship, entity) in (&space_ships, &entities).join() {
        //     if space_ship.is_attacking && space_ship.last_attack_time + space_ship.attack_cooldown <= time.absolute_timeseconds() {
        //         let transform = transforms.get(entity).unwrap().clone();
        //         let sprite_render = SpriteRender {
        //             sprite_sheet: sprite_sheet_handle.clone(),
        //             sprite_number: 5
        //         };
        //         let is_player = player_ship_tags.contains(entity);
        //         let direction_y = if is_player {
        //             1.0
        //         } else {
        //             -1.0
        //         };

        //         entities
        //             .build_entity()
        //             .with(transform, &mut transforms)
        //             .with(sprite_render, &mut sprite_renders)
        //             .with(Missile::new(is_player), &mut missiles)
        //             .with(
        //                 Moveable {
        //                     move_speed: 500.0,
        //                     direction: Vector2::new(0.0, direction_y)
        //                 }, 
        //                 &mut moveables
        //             )
        //             .with(
        //                 Rect {
        //                     width: constants::MISSILE_WIDTH,
        //                     height: constants::MISSILE_HEIGHT
        //                 },
        //                 &mut rects
        //             )
        //             .with(DestroyOutOfArenaTag, &mut destroy_out_of_arena_tags)
        //             .build();

        //         space_ship.last_attack_time = time.absolute_time_seconds();
        //     }
        // }        
    }
}