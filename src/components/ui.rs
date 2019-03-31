use amethyst::{
    ecs::{
        Component,
        DenseVecStorage,
        world::Index
    }
};

pub struct UiKillable {
    pub last_health: i32,
    pub health_segment_entities: Vec<Index>
}

impl Component for UiKillable {
    type Storage = DenseVecStorage<Self>;
}

