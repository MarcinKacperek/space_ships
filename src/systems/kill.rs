use amethyst::{
    ecs::{
        Entities,
        Join,
        WriteExpect,
        WriteStorage,
        ReadExpect,
        ReadStorage,
        System
    },
    // prelude::*,
    ui::UiText
};
use crate::{
    components::{
        Killable,
        tags::PlayerShipTag,
        data::{
            GameplaySessionData,
            UiGameplayElements
        }
    },
    resources::{
        GameplayNextState,
        GameState
    }
};

pub struct KillSystem;

impl<'s> System<'s> for KillSystem {
    type SystemData = (
        WriteStorage<'s, Killable>,
        ReadStorage<'s, PlayerShipTag>,
        WriteStorage<'s, UiText>,
        WriteExpect<'s, GameplaySessionData>,
        ReadExpect<'s, UiGameplayElements>,
        WriteExpect<'s, GameplayNextState>,
        Entities<'s>
    );

    fn run(&mut self, (mut killables, player_ship_tags, mut ui_texts, mut session_data, ui_elements, mut gameplay_next_state, entities): Self::SystemData) {
        for (killable, entity) in (&mut killables, &entities).join() {
            if entities.is_alive(entity) && !killable.is_alive() {
                // TODO add score to enemy
                session_data.score += 1;
                if let Some(text) = ui_texts.get_mut(ui_elements.score_value_text) {
                    text.text = session_data.score.to_string();
                }

                let _ = entities.delete(entity);

                if player_ship_tags.contains(entity) {
                    gameplay_next_state.next_state = Some(GameState::Finished);
                }
            }
        }

        // TODO add game lost on player hp == 0
    }
}