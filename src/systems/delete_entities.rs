use amethyst::ecs::{
    Join,
    ReadStorage,
    Entities,
    System
};
use crate::components::tags::DeleteEntityTag;

pub struct DeleteEntitiesSystem;

impl<'s> System<'s> for DeleteEntitiesSystem {
    type SystemData = (
        ReadStorage<'s, DeleteEntityTag>,
        Entities<'s>
    );

    fn run(&mut self, (delete_entity_tags, entities): Self::SystemData) {
        for (_, entity) in (&delete_entity_tags, &entities).join() {
            let _ = entities.delete(entity);
        }
    }
}