use amethyst::{
    ecs::{
        Entities,
        Join,
        WriteExpect,
        WriteStorage,
        ReadStorage,
        System
    }
};
use crate::{
    components::{
        Killable,
        tags::PlayerShipTag,
        data::GameplaySessionData
    }
};

pub struct KillSystem;

impl<'s> System<'s> for KillSystem {
    type SystemData = (
        WriteStorage<'s, Killable>,
        ReadStorage<'s, PlayerShipTag>,
        WriteExpect<'s, GameplaySessionData>,
        Entities<'s>
    );

    fn run(&mut self, (mut killables, player_ship_tags, mut session_data, entities): Self::SystemData) {
        for (killable, entity, _) in (&mut killables, &entities, !&player_ship_tags).join() {
            if entities.is_alive(entity) && !killable.is_alive() {
                // TODO add score to enemy
                session_data.score += 1;
                let _ = entities.delete(entity);
            }
        }

        // TODO add game lost on player hp == 0
    }
}