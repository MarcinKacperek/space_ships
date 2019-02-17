use amethyst::{
    core::Transform,
    ecs::{
        Join,
        WriteStorage,
        ReadStorage,
        Entities,
        System
    }
};
use crate::{
    components::{
        Rect,
        tags::DestroyOutOfArenaTag
    },
    constants
};

pub struct DestroyOutOfArenaSystem;

impl<'s> System<'s> for DestroyOutOfArenaSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Rect>,
        ReadStorage<'s, DestroyOutOfArenaTag>
    );

    fn run(&mut self, (entities, mut transforms, rects, destroy_out_of_arena_tags): Self::SystemData) {
        for (entity, transform, rect, _) in (&entities, &mut transforms, &rects, &destroy_out_of_arena_tags).join() {
            // Not using halves, it's ok to let entities leave arena a bit further
            let x = transform.translation().x;
            let y = transform.translation().y;
            if
                x < -rect.width ||
                x > constants::ARENA_WIDTH + rect.width ||
                y < -rect.height ||
                y > constants::ARENA_HEIGHT + rect.height 
            {
                let _ = entities.delete(entity);
            }
        }
    }
}







// use amethyst::{
//     core::Transform,
//     ecs::{
//         Join,
//         ReadStorage,
//         WriteStorage,
//         System
//     }
// };

// use crate::{
//     components::{
//         Rect,
//         tags::BoundInArenaTag
//     },
//     constants
// };

// pub struct MovementSystem;

// impl<'s> System<'s> for MovementSystem {
//     type SystemData = (
//         ReadStorage<'s, Moveable>,
//         WriteStorage<'s, Transform>,
//         Read<'s, Time>
//     );

//     fn run(&mut self, (moveables, mut transforms, time): Self::SystemData) {
//         for (moveable, transform) in (&moveables, &mut transforms).join() {
//             let movement = moveable.direction * moveable.move_speed * time.delta_seconds();
//             transform.translate_xyz(movement.x, movement.y, 0.0);
//         }
//     }
// }