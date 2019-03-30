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
use crate::components::{
    Killable,
    Rect,
    tags::{
        EnemyTag,
        DeleteEntityTag,
        PlayerShipTag
    }
};


pub struct EnemyCollisionSystem;

// impl<'s> System<'s> for EnemyCollisionSystem {
//     type SystemData = (
//         ReadStorage<'s, Transform>,
//         ReadStorage<'s, Rect>,
//         ReadStorage<'s, PlayerShipTag>,
//         ReadStorage<'s, EnemyTag>,
//         WriteStorage<'s, Killable>,
//         WriteStorage<'s, DeleteEntityTag>,
//     );
// }