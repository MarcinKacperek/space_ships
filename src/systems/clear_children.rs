use amethyst::{
    ecs::{
        Entities,
        Join,
        ReadStorage,
        WriteStorage,
        System
    }
};
use crate::components::{
    Killable,
    SpaceShip,
    tags::{
        DeleteEntityTag,
        PlayerShipTag
    },
    ui::UiKillable
};

pub struct ClearChildrenSystem;

impl<'s> System<'s> for ClearChildrenSystem {
    type SystemData = (
        ReadStorage<'s, Killable>,
        ReadStorage<'s, SpaceShip>,
        ReadStorage<'s, PlayerShipTag>,
        ReadStorage<'s, UiKillable>,
        WriteStorage<'s, DeleteEntityTag>,
        Entities<'s>
    );

    fn run(
        &mut self,
        (
            killables,
            space_ships,
            player_ship_tags,
            ui_killables,
            mut delete_entity_tags,
            entities
        ): Self::SystemData
    ) {
        // Clear healthbars
        for (killable, entity, _) in (&killables, &entities, !&player_ship_tags).join() {
            if delete_entity_tags.contains(entity) {
                if let Some(health_bar_entity_index) = killable.health_bar_entity_index {
                    let health_bar_entity = entities.entity(health_bar_entity_index);
                    let _ = delete_entity_tags.insert(health_bar_entity, DeleteEntityTag);
                    let ui_killable = ui_killables.get(health_bar_entity).unwrap();
                    for health_segment_index in &ui_killable.health_segment_entities {
                        let health_segment_entity = entities.entity(*health_segment_index);
                        let _ = delete_entity_tags.insert(health_segment_entity, DeleteEntityTag);
                    }
                }
            }
        }
        // Clear cannons
        for (space_ship, entity) in (&space_ships, &entities).join() {
            if delete_entity_tags.contains(entity) {
                for cannon_entity_index in &space_ship.cannon_entities_indices {
                    let cannon_entity = entities.entity(*cannon_entity_index);
                    let _ = delete_entity_tags.insert(cannon_entity, DeleteEntityTag);
                }
            }
        }
    }

}