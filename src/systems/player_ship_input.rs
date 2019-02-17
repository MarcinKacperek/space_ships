use amethyst::{
    ecs::{
        Join,
        Read,
        ReadStorage,
        WriteStorage,
        System
    },
    input::InputHandler
};

use crate::components::{
    Moveable,
    SpaceShip,
    tags::{
        PlayerShipTag
    }
};

pub struct PlayerShipSystem;

impl<'s> System<'s> for PlayerShipSystem {
    type SystemData = (
        WriteStorage<'s, Moveable>,
        WriteStorage<'s, SpaceShip>,
        ReadStorage<'s, PlayerShipTag>,
        Read<'s, InputHandler<String, String>>
    );

    fn run(&mut self, (mut moveables, mut space_ships, player_ship, input): Self::SystemData) {
        for (moveable, space_ship, _) in (&mut moveables, &mut space_ships, &player_ship).join() {
            let x_movement = input.axis_value("x_axis").unwrap_or(0.0);
            let y_movement = input.axis_value("y_axis").unwrap_or(0.0);

            moveable.direction.x = x_movement as f32;
            moveable.direction.y = y_movement as f32;
            if moveable.direction.magnitude() != 0.0 {
                moveable.direction.normalize_mut();
            }

            // Whether to shoot next frame
            space_ship.is_attacking = input.action_is_down("fire").unwrap_or(false);
        }
    }
}