use amethyst::{
    core::{
        Transform,
        Time
    },
    ecs:: {
        Entities,
        Read,
        ReadExpect,
        WriteStorage,
        System
    },
    renderer::{
        SpriteRender,
        SpriteSheetHandle
    }
};
use rand::prelude::*;
use crate::{
    components::{
        Rect,
        Moveable,
        SpaceShip,
        Killable,
        tags::{
            EnemyTag,
            DestroyOutOfArenaTag
        }
    },
    constants,
    prefabs::{
        EnemyPrefabData,
        EnemyPrefabs,
        SimplePrefab
    }
};

#[derive(Default)]
pub struct EnemySpawnerSystem {
    next_spawn_time: f64
}

impl EnemySpawnerSystem {

    fn get_random_enemy_prefab<'a>(enemy_prefabs: &'a Vec<EnemyPrefabData>, rng: &mut ThreadRng) -> &'a EnemyPrefabData {
        let idx = rng.gen_range(0, enemy_prefabs.len());
        return & enemy_prefabs.get(idx).unwrap();
    }

}

impl<'s> System<'s> for EnemySpawnerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Rect>,
        WriteStorage<'s, Moveable>,
        WriteStorage<'s, Killable>,        
        WriteStorage<'s, SpaceShip>,
        WriteStorage<'s, EnemyTag>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, DestroyOutOfArenaTag>,
        ReadExpect<'s, SpriteSheetHandle>,
        ReadExpect<'s, EnemyPrefabs>,
        Read<'s, Time>,
        Entities<'s>
    );

    fn run(
        &mut self, 
        (
            transforms,
            rects,
            moveables,
            killables,
            space_ships,
            enemy_tags,
            sprite_renders,
            destroy_out_of_arena_tags,
            sprite_sheet_handle,
            enemy_prefabs,
            time,
            entities
        ): Self::SystemData
    ) {
        if self.next_spawn_time <= time.absolute_time_seconds() {
            let mut rng = rand::thread_rng();
            let enemy_type: f64 = rng.gen();
            let enemy_prefab = {
                // TODO put enemy type chances somewhere else
                // TOOD confusing and ugly, refactor
                if enemy_type > 0.1 && enemy_prefabs.large_enemy_count() > 0 { // 10% chance
                    Self::get_random_enemy_prefab(&enemy_prefabs.large_enemy_prefabs, &mut rng)
                } else if enemy_type > 0.4 && enemy_prefabs.medium_enemy_count() > 0 { // 30% chance
                    Self::get_random_enemy_prefab(&enemy_prefabs.medium_enemy_prefabs, &mut rng)
                } else if enemy_prefabs.small_enemy_count() > 0 { // 60% chance
                    Self::get_random_enemy_prefab(&enemy_prefabs.small_enemy_prefabs, &mut rng)
                } else {
                    panic!("enemy_spawner, no enemy prefabs were loaded!");
                }
            };

            // Position
            let width = &enemy_prefab.width;
            let height = &enemy_prefab.height;
            let x = rng.gen_range(width / 2.0, constants::ARENA_WIDTH - width / 2.0);
            // -1.0 so it's not deleted by out of bounds system
            let y = constants::ARENA_HEIGHT + height - 1.0; 

            // Spawn enemy
            enemy_prefab.create_entity(
                &entities, 
                x,
                y,
                &mut (
                    transforms,
                    rects,
                    moveables,
                    killables,
                    space_ships,
                    enemy_tags,
                    sprite_renders,
                    destroy_out_of_arena_tags,
                    sprite_sheet_handle
                )
            );

            // Update next spawn time
            let next_spawn_delay = rand::thread_rng().gen_range(constants::ENEMY_SPAWNER_MIN_DELAY, constants::ENEMY_SPAWNER_MAX_DELAY);
            self.next_spawn_time = time.absolute_time_seconds() + next_spawn_delay;
        }
    }

}