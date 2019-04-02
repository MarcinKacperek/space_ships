use amethyst::{
    core::{
        Time
    },
    ecs::{
        Entities,
        Join,
        Read,
        ReadStorage,
        System,
        WriteStorage
    }
};
use crate::{
    components::{
        Expire,
        tags::DeleteEntityTag
    }
};

pub struct ExpireSystem;

impl<'s> System<'s> for ExpireSystem {
    type SystemData = (
        ReadStorage<'s, Expire>,
        WriteStorage<'s, DeleteEntityTag>,
        Entities<'s>,
        Read<'s, Time>
    );

    fn run(
        &mut self,
        (
            expires,
            mut delete_entity_tags,
            entities,
            time
        ): Self::SystemData
    ) {
        let current_time = time.absolute_time_seconds();
        for (expire, entity) in (&expires, &entities).join() {
            if expire.is_expired(current_time) {
                let _ = delete_entity_tags.insert(entity, DeleteEntityTag);
            }
        }
    }

}