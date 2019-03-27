use amethyst::{
    ecs::{
        Entity,
        SystemData
    }
};

mod enemy_prefab;

pub use {
    enemy_prefab::EnemyPrefabData
};

pub struct EnemyPrefabs {
    pub small_enemy_prefabs: Vec<EnemyPrefabData>,
    pub medium_enemy_prefabs: Vec<EnemyPrefabData>,
    pub large_enemy_prefabs: Vec<EnemyPrefabData>
}

impl EnemyPrefabs {

    pub fn small_enemy_count(&self) -> usize {
        return self.small_enemy_prefabs.len();
    }

    pub fn medium_enemy_count(&self) -> usize {
        return self.medium_enemy_prefabs.len();
    }

    pub fn large_enemy_count(&self) -> usize {
        return self.large_enemy_prefabs.len();
    }

}

pub trait SimplePrefab<'a> {
    type SystemData: SystemData<'a>;

    fn add_to_entity(&self, entity: Entity, x: f32, y: f32, system_data: &mut Self::SystemData);

}