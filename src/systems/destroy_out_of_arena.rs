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
        tags::{
            DeleteEntityTag,
            DestroyOutOfArenaTag
        }
    },
    constants
};

pub struct DestroyOutOfArenaSystem;

impl<'s> System<'s> for DestroyOutOfArenaSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Rect>,
        ReadStorage<'s, DestroyOutOfArenaTag>,
        WriteStorage<'s, DeleteEntityTag>
    );

    fn run(
        &mut self, 
        (
            entities, 
            mut transforms, 
            rects, 
            destroy_out_of_arena_tags,
            mut delete_entity_tags
        ): Self::SystemData
    ) {
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
                let _ = delete_entity_tags.insert(entity, DeleteEntityTag);
            }
        }
    }
}