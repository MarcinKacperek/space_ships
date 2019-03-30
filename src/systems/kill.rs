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
        tags::{
            DeleteEntityTag,
            PlayerShipTag
        }
    },
    resources::{
        GameplayNextState,
        GameplaySessionData,
        GameState,
    }
};

pub struct KillSystem;

impl<'s> System<'s> for KillSystem {
    type SystemData = (
        WriteStorage<'s, Killable>,
        ReadStorage<'s, PlayerShipTag>,
        WriteStorage<'s, DeleteEntityTag>,
        WriteExpect<'s, GameplaySessionData>,
        WriteExpect<'s, GameplayNextState>,
        Entities<'s>
    );

    fn run(
        &mut self, 
        (
            mut killables, 
            player_ship_tags, 
            mut delete_entity_tags,
            mut session_data, 
            mut gameplay_next_state, 
            entities
        ): Self::SystemData
    ) {
        for (killable, entity) in (&mut killables, &entities).join() {
            if !delete_entity_tags.contains(entity) && !killable.is_alive() {
                if !player_ship_tags.contains(entity) {
                    // TODO add score to enemy
                    session_data.score += 1;
                } else {
                    gameplay_next_state.next_state = Some(GameState::Finished);
                }

                let _ = delete_entity_tags.insert(entity, DeleteEntityTag);
            }
        }
    }
}