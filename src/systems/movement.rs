use amethyst::{
    core::{
        Transform,
        Time
    },
    ecs::{
        Join,
        Read,
        ReadStorage,
        WriteStorage,
        System
    }
};
use crate::{
    components::{
        Moveable
    }
};

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, Moveable>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>
    );

    fn run(&mut self, (moveables, mut transforms, time): Self::SystemData) {
        for (moveable, transform) in (&moveables, &mut transforms).join() {
            let movement = moveable.direction * moveable.move_speed * time.delta_seconds();
            transform.translate_xyz(movement.x, movement.y, 0.0);
        }
    }
}