use amethyst::{
    core::{
        Parent,
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
        Cannon,
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
pub struct CannonPrefabData {
    pub x_offset: f32,
    pub y_offset: f32,
    pub missile_width: f32,
    pub missile_height: f32,
    pub missile_speed: f32,
    pub missile_sprite_index: usize
}

#[derive(Serialize, Deserialize)]
pub struct EnemyPrefabData {
    pub sprite_index: usize,
    pub movement_speed_min: f32,
    pub movement_speed_max: f32,
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub health: i32,
    pub drops_health: bool,
    pub attack_cooldown: Option<f64>,
    pub cannon_prefabs: Option<Vec<CannonPrefabData>>
}

impl<'a> SimplePrefab<'a> for EnemyPrefabData {
    type SystemData = (
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Rect>,
        WriteStorage<'a, Moveable>,
        WriteStorage<'a, Killable>,
        WriteStorage<'a, SpaceShip>,
        WriteStorage<'a, Cannon>,
        WriteStorage<'a, EnemyTag>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, DestroyOutOfArenaTag>,
        WriteStorage<'a, Parent>,
        ReadExpect<'a, SpriteSheetHandle>
    );

    fn init(&mut self) {
        // Apply scale to all dimensions
        self.width = self.width * self.scale;
        self.height = self.height * self.scale;

        if let Some(cannon_prefabs) = &mut self.cannon_prefabs {
            for cannon in cannon_prefabs {
                cannon.x_offset = cannon.x_offset * self.scale;
                cannon.y_offset = cannon.y_offset * self.scale;
                cannon.missile_width = cannon.missile_width * self.scale;
                cannon.missile_height = cannon.missile_height * self.scale;
            }
        }
    }

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
            ref mut cannons,
            ref mut enemy_tags,
            ref mut sprite_renders,
            ref mut destroy_out_of_arena_tags,
            ref mut parents,
            sprite_sheet_handle
        ): &mut Self::SystemData
    ) {
        let enemy_entity = entities.create();

        let mut transform = Transform::default();
        transform.set_xyz(x, y, 0.0);
        transform.set_scale(self.scale, self.scale, 1.0);

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
            .insert(enemy_entity, Killable::new_enemy(self.health, self.drops_health))
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
        space_ships
            .insert(enemy_entity, SpaceShip {
                is_attacking: true,
                cannon_entities_indices: Vec::new()
            })
            .expect("Could not create SpaceShip!");

        let enemy_space_ship = space_ships.get_mut(enemy_entity).unwrap();
        // Create cannons
        if let Some(cannon_prefabs) = &self.cannon_prefabs {
            let attack_cooldown = self.attack_cooldown
                .expect("Attack cooldown is required if cannons are specified!");

            for cannon_prefab in cannon_prefabs {
                let cannon_entity = entities.create();
                enemy_space_ship.cannon_entities_indices.push(cannon_entity.id());
                cannons
                    .insert(cannon_entity, Cannon {
                        x_offset: cannon_prefab.x_offset,
                        y_offset: cannon_prefab.y_offset,
                        attack_cooldown: attack_cooldown,
                        last_attack_time: 0.0,
                        missile_width: cannon_prefab.missile_width,
                        missile_height: cannon_prefab.missile_height,
                        missile_speed: cannon_prefab.missile_speed,
                        missile_sprite_index: cannon_prefab.missile_sprite_index
                    })
                    .expect("Could not create Cannon!");
                parents
                    .insert(cannon_entity, Parent {
                        entity: enemy_entity
                    })
                    .expect("Could not create Parent!");
            }
        }
    }

}