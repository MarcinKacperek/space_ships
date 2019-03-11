use amethyst::{
    core::{
        Transform,
        Time,
        nalgebra::Vector2
    },
    ecs:: {
        Entities,
        Read,
        ReadExpect,
        WriteStorage,
        System
    },
    renderer::{
        Flipped,
        SpriteRender,
        SpriteSheetHandle
    }
};
use rand::prelude::*;
use crate::{
    components::{
        Rect,
        Moveable,
        SpaceShip,
        Killable,
        tags::{
            EnemyTag,
            DestroyOutOfArenaTag
        }
    },
    constants
};

#[derive(Default)]
pub struct EnemySpawnerSystem {
    next_spawn_time: f64
}

impl<'s> System<'s> for EnemySpawnerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Rect>,
        WriteStorage<'s, Moveable>,
        WriteStorage<'s, Killable>,        
        WriteStorage<'s, SpaceShip>,
        WriteStorage<'s, EnemyTag>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, DestroyOutOfArenaTag>,
        WriteStorage<'s, Flipped>,
        ReadExpect<'s, SpriteSheetHandle>,
        Read<'s, Time>,
        Entities<'s>
    );

    fn run(
        &mut self, 
        (
            mut transforms,
            mut rects,
            mut moveables,
            mut killables,
            mut space_ships,
            mut enemy_tags,
            mut sprite_renders,
            mut destroy_out_of_arena_tags,
            mut flipped,
            sprite_sheet_handle,
            time,
            entities
        ): Self::SystemData
    ) {
        if self.next_spawn_time <= time.absolute_time_seconds() {
            let mut transform = Transform::default();
            let x = rand::thread_rng().gen_range(constants::ENEMY_SPAWNER_POINT_MIN_X, constants::ENEMY_SPAWNER_POINT_MAX_X);
            transform.set_xyz(x, constants::ENEMY_SPAWNER_POINT_Y, 0.0);

            // TODO: Refactor enemy type -> change to enum
            let enemy_type = rand::thread_rng().gen_range(1, 5);

            let sprite_render = SpriteRender {
                sprite_sheet: sprite_sheet_handle.clone(),
                sprite_number: enemy_type
            };

            let moveable = Moveable {
                move_speed: rand::thread_rng().gen_range(50.0, 175.0),
                direction: Vector2::new(0.0, -1.0)
            };

            let killable = Killable::new(2);

            let rect = Rect {
                width: constants::ENEMY_WIDTH,
                height: constants::ENEMY_HEIGHT
            };

            if enemy_type == 1 {
                entities
                    .build_entity()
                    .with(transform, &mut transforms)
                    .with(sprite_render, &mut sprite_renders)
                    .with(Flipped::Vertical, &mut flipped)
                    .with(moveable, &mut moveables)
                    .with(killable, &mut killables)
                    .with(rect, &mut rects)
                    .with(DestroyOutOfArenaTag, &mut destroy_out_of_arena_tags)
                    .with(EnemyTag, &mut enemy_tags)
                    .build();
            } else {
                entities
                    .build_entity()
                    .with(transform, &mut transforms)
                    .with(sprite_render, &mut sprite_renders)
                    .with(Flipped::Vertical, &mut flipped)
                    .with(moveable, &mut moveables)
                    .with(killable, &mut killables)
                    .with(rect, &mut rects)
                    .with(DestroyOutOfArenaTag, &mut destroy_out_of_arena_tags)
                    .with(EnemyTag, &mut enemy_tags)
                    .with(
                        SpaceShip {
                            attack_cooldown: 3.0,
                            last_attack_time: 0.0,
                            is_attacking: true
                        }, 
                        &mut space_ships
                    )
                    .build();
            }

            // Update next spawn time
            let next_spawn_delay = rand::thread_rng().gen_range(constants::ENEMY_SPAWNER_MIN_DELAY, constants::ENEMY_SPAWNER_MAX_DELAY);
            self.next_spawn_time = time.absolute_time_seconds() + next_spawn_delay;
        }
    }

}