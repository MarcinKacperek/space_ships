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
            EnemyTag,
            DeleteEntityTag,
            PlayerShipTag
        }
    },
    utils
};

pub struct EnemyCollisionSystem;

impl<'s> System<'s> for EnemyCollisionSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Rect>,
        ReadStorage<'s, PlayerShipTag>,
        ReadStorage<'s, EnemyTag>,
        WriteStorage<'s, Killable>,
        WriteStorage<'s, DeleteEntityTag>,
        Entities<'s>
    );

    fn run(
        &mut self,
        (
            transforms,
            rects,
            player_ship_tags,
            enemy_tags,
            mut killables,
            mut delete_entity_tags,
            entities
        ): Self::SystemData
    ) {
        let (
            player_transform, 
            player_rect, 
            player_killable, 
            _
        ) = (&transforms, &rects, &mut killables, &player_ship_tags).join().last().unwrap();

        for (enemy_transform, enemy_rect, enemy_entity, _) in (&transforms, &rects, &entities, &enemy_tags).join() {
            if !delete_entity_tags.contains(enemy_entity) && utils::is_aabb_collide(player_rect, player_transform, enemy_rect, enemy_transform) {
                player_killable.deal_damage();
                let _ = delete_entity_tags.insert(enemy_entity, DeleteEntityTag);
            }
        }

    }
}