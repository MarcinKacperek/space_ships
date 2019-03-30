use amethyst::{
    ecs::{
        Join,
        ReadExpect,
        ReadStorage,
        System,
        WriteStorage
    },
    ui::{
        UiText
    }
};
use crate::{
    components::{
        Killable,
        tags::{
            PlayerShipTag
        }
    },
    resources::{
        GameplaySessionData,
        UiGameplayElements
    }
};

pub struct UiSystem;

impl UiSystem {

    fn update_score<'s>(
        gameplay_session_data: &ReadExpect<'s, GameplaySessionData>,
        ui_gameplay_elements: &ReadExpect<'s, UiGameplayElements>,
        ui_texts: &mut WriteStorage<'s, UiText>
    ) {
        if let Some(text) = ui_texts.get_mut(ui_gameplay_elements.score_value_text) {
            text.text = gameplay_session_data.score.to_string();
        }
    }

    fn update_player_lives<'s>(
        killables: &ReadStorage<'s, Killable>,
        player_ship_tags: &ReadStorage<'s, PlayerShipTag>,
        ui_texts: &mut WriteStorage<'s, UiText>,
        ui_gameplay_elements: &ReadExpect<'s, UiGameplayElements>
    ) {
        for (player_killable, _) in (killables, player_ship_tags).join() {
            if let Some(text) = ui_texts.get_mut(ui_gameplay_elements.life_value_text) {
                text.text = player_killable.get_health().to_string();
            }
        }
    }

    // Handle health bars
    // Handle ammo bar

}

impl<'s> System<'s> for UiSystem {
    type SystemData = (
        ReadStorage<'s, Killable>,
        ReadStorage<'s, PlayerShipTag>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, GameplaySessionData>,
        ReadExpect<'s, UiGameplayElements>
    );

    fn run(
        &mut self,
        (
            killables,
            player_ship_tags,
            mut ui_texts,
            gameplay_session_data,
            ui_gameplay_elements
        ): Self::SystemData 
    ) {
        UiSystem::update_player_lives(
            &killables, 
            &player_ship_tags, 
            &mut ui_texts, 
            &ui_gameplay_elements
        );
        UiSystem::update_score(
            &gameplay_session_data, 
            &ui_gameplay_elements,
            &mut ui_texts
        );
    }

}