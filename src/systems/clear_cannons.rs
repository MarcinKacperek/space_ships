use amethyst::{
    core::{
        Parent
    },
    ecs::{
        Entities,
        Join,
        ReadStorage,
        System,
        WriteStorage
    }
};
use crate::components::{
    Cannon,
    tags::DeleteEntityTag
};


pub struct ClearCannonsSystem;

impl<'s> System<'s> for ClearCannonsSystem {
    type SystemData = (
        ReadStorage<'s, Cannon>,
        ReadStorage<'s, Parent>,
        WriteStorage<'s, DeleteEntityTag>,
        Entities<'s>
    );

    fn run(
        &mut self,
        (
            cannons,
            parents,
            mut delete_entity_tags,
            entities
        ): Self::SystemData
    ) {
        for (_, parent, cannon_entity) in (&cannons, &parents, &entities).join() {
            if delete_entity_tags.contains(parent.entity) {
                let _ = delete_entity_tags.insert(cannon_entity, DeleteEntityTag);
            }
        }
    }
}