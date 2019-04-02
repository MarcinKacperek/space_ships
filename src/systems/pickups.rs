use amethyst::{
    core::Transform,
    ecs::{
        Entities,
        Join,
        ReadStorage,
        System,
        WriteStorage
    }
};
use crate::{
    components::{
        Killable,
        Rect,
        tags::{
            DeleteEntityTag,
            HealthPickupTag,
            PlayerShipTag
        }
    },
    utils
};

pub struct PickupsSystem;

impl<'s> System<'s> for PickupsSystem {
    type SystemData = (
        ReadStorage<'s, HealthPickupTag>,
        ReadStorage<'s, PlayerShipTag>,
        ReadStorage<'s, Rect>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, DeleteEntityTag>,
        WriteStorage<'s, Killable>,
        Entities<'s>
    );

    fn run(
        &mut self,
        (
            health_pickup_tags,
            player_ship_tags,
            rects,
            transforms,
            mut delete_entity_tags,
            mut killables,
            entities
        ): Self::SystemData
    ) {
        for (player_rect, player_transform, player_killable, _) in (&rects, &transforms, &mut killables, &player_ship_tags).join() {
            for (pickup_rect, pickup_transform, pickup_entity, _) in (&rects, &transforms, &entities, &health_pickup_tags).join() {
                if !delete_entity_tags.contains(pickup_entity) && utils::is_aabb_collide(player_rect, player_transform, pickup_rect, pickup_transform) {
                    player_killable.gain_health();
                    let _ = delete_entity_tags.insert(pickup_entity, DeleteEntityTag);
                }
            }
        }

    }

}