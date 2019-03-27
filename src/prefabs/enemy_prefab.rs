use amethyst::{
    core::{
        Transform,
        nalgebra::Vector2
    },
    ecs::{
        Entities,
        ReadExpect,
        WriteStorage
    },
    renderer::{
        SpriteRender,
        SpriteSheetHandle
    }
};
use crate::{
    components::{
        Killable,
        Moveable,
        Rect,
        SpaceShip,
        tags::{
            DestroyOutOfArenaTag,
            EnemyTag
        }
    },
    prefabs::SimplePrefab
};
use rand::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct EnemyPrefabData {
    pub sprite_index: usize,
    pub movement_speed_min: f32,
    pub movement_speed_max: f32,
    pub width: f32,
    pub height: f32,
    pub health: i32,
    pub attack_cooldown: Option<f64>
}

impl<'a> SimplePrefab<'a> for EnemyPrefabData {
    type SystemData = (
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Rect>,
        WriteStorage<'a, Moveable>,
        WriteStorage<'a, Killable>,
        WriteStorage<'a, SpaceShip>,
        WriteStorage<'a, EnemyTag>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, DestroyOutOfArenaTag>,
        ReadExpect<'a, SpriteSheetHandle>
    );

    fn create_entity(
        &self, 
        entities: &'a Entities, 
        x: f32,
        y: f32,
        (
            ref mut transforms,
            ref mut rects,
            ref mut moveables,
            ref mut killables,
            ref mut space_ships,
            ref mut enemy_tags,
            ref mut sprite_renders,
            ref mut destroy_out_of_arena_tags,
            sprite_sheet_handle
        ): &mut Self::SystemData
    ) {
        let enemy_entity = entities.create();

        let mut transform = Transform::default();
        transform.set_xyz(x, y, 0.0);

        transforms
            .insert(enemy_entity, transform)
            .expect("Could not create Transform!");
        rects
            .insert(enemy_entity, Rect {
                width: self.width,
                height: self.height
            })
            .expect("Could not create Rect!");
        moveables
            .insert(enemy_entity, Moveable {
                move_speed: rand::thread_rng().gen_range(self.movement_speed_min, self.movement_speed_max),
                direction: Vector2::new(0.0, -1.0)
            })
            .expect("Could not create Moveable!");
        killables
            .insert(enemy_entity, Killable::new(self.health))
            .expect("Could not create Killable!");
        enemy_tags
            .insert(enemy_entity, EnemyTag)
            .expect("Could not create EnemyTag!");
        destroy_out_of_arena_tags
            .insert(enemy_entity, DestroyOutOfArenaTag)
            .expect("Could not create DestroyOutOfArenaTag!");
        sprite_renders
            .insert(enemy_entity, SpriteRender {
                sprite_sheet: sprite_sheet_handle.clone(),
                sprite_number: self.sprite_index
            })
            .expect("Could not create SpriteRender!");

        if let Some(attack_cooldown) = self.attack_cooldown {
            space_ships
                .insert(enemy_entity, SpaceShip {
                    attack_cooldown: attack_cooldown,
                    last_attack_time: 0.0,
                    is_attacking: true
                })
                .expect("Could not create SpaceShip!");
        }

        // Create cannons
    }

}