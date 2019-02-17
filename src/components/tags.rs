use amethyst::ecs::{
    Component,
    NullStorage
};

#[derive(Default)]
pub struct PlayerShipTag;

impl Component for PlayerShipTag {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct EnemyTag;

impl Component for EnemyTag {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct BoundInArenaTag;

impl Component for BoundInArenaTag {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct DestroyOutOfArenaTag;

impl Component for DestroyOutOfArenaTag {
    type Storage = NullStorage<Self>;
}