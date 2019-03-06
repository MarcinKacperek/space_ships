use amethyst::{
    core::Transform,
    ecs::{
        Join,
        Entities,
        ReadExpect,
        ReadStorage,
        WriteStorage,
        System,
    },
    ui::UiText
};
use crate::{
    components::{
        Rect,
        Missile,
        Killable,
        tags::{
            PlayerShipTag,
            EnemyTag
        },
        data::UiGameplayElements
    },
    utils
};

pub struct MissileSystem;

impl<'s> System<'s> for MissileSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Rect>,
        ReadStorage<'s, Missile>,
        WriteStorage<'s, Killable>,
        ReadStorage<'s, PlayerShipTag>,
        ReadStorage<'s, EnemyTag>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, UiGameplayElements>,
        Entities<'s>
    );

    fn run(&mut self, (transforms, rects, missiles, mut killables, player_ship_tags, enemy_tags, mut ui_texts, ui_elements, entities): Self::SystemData) {
        for (missile_transform, missile_rect, missile, missile_entity) in (&transforms, &rects, &missiles, &entities).join() {
            if missile.belongs_to_player() {
                for (enemy_transform, enemy_rect, enemy_killable, _) in (&transforms, &rects, &mut killables, &enemy_tags).join() {
                    // Missile could have already collided with something during this frame
                    if !entities.is_alive(missile_entity) {
                        break;
                    }
                    // Enemy could have already died
                    if 
                        enemy_killable.is_alive() &&
                        utils::is_aabb_collide(missile_rect, missile_transform, enemy_rect, enemy_transform) 
                    {
                        // TODO: Explosion animation
                        enemy_killable.deal_damage(missile.get_damage());
                        let _ = entities.delete(missile_entity);
                    }
                }
            } else {
                for (player_transform, player_rect, player_killable, _) in (&transforms, &rects, &mut killables, &player_ship_tags).join() {
                    // Missile could have already collided with something during this frame
                    if !entities.is_alive(missile_entity) {
                        break;
                    }
                    if
                        player_killable.is_alive() &&
                        utils::is_aabb_collide(missile_rect, missile_transform, player_rect, player_transform)
                    {
                        player_killable.deal_damage(missile.get_damage());
                        if let Some(text) = ui_texts.get_mut(ui_elements.life_value_text) {
                            text.text = player_killable.get_health().to_string();
                        }


                        let _ = entities.delete(missile_entity);
                    }
                }
            }
        } 
    }
}