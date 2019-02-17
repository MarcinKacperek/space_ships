use amethyst::{
    ecs::{
        Entities,
        Join,
        WriteStorage,
        ReadStorage,
        System
    }
};
use crate::{
    components::{
        Killable,
        tags::{
            // EnemyTag,
            PlayerShipTag
        }
    }
};

pub struct KillSystem;

impl<'s> System<'s> for KillSystem {
    type SystemData = (
        WriteStorage<'s, Killable>,
        // ReadStorage<'s, EnemyTag>,
        ReadStorage<'s, PlayerShipTag>,
        Entities<'s>
    );

    fn run(&mut self, (mut killables, /*enemy_tags,*/ player_ship_tags, entities): Self::SystemData) {
        for (killable, entity, _) in (&mut killables, &entities, !&player_ship_tags).join() {
            if entities.is_alive(entity) && !killable.is_alive() {
                let _ = entities.delete(entity);
            }
        }
    }
}