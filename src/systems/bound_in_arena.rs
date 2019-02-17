use amethyst::{
    core::Transform,
    ecs::{
        Join,
        ReadStorage,
        WriteStorage,
        System
    }
};

use crate::{
    components::{
        Rect,
        tags::BoundInArenaTag
    },
    constants
};

pub struct BoundInArenaSystem;

impl<'s> System<'s> for BoundInArenaSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Rect>,
        ReadStorage<'s, BoundInArenaTag>
    );

    fn run(&mut self, (mut transforms, rects, bound_in_arena_tags): Self::SystemData) {
        for (transform, rect, _) in (&mut transforms, &rects, &bound_in_arena_tags).join() {
            let half_rect_width = rect.width / 2.0;
            let half_rect_height = rect.height / 2.0;

            let x = transform.translation().x;
            if x < half_rect_width {
                transform.set_x(half_rect_width);
            } else if x > constants::ARENA_WIDTH - half_rect_width {
                transform.set_x(constants::ARENA_WIDTH - half_rect_width);
            }
        
            let y = transform.translation().y;
            if y < half_rect_height {
                transform.set_y(half_rect_height);
            } else if y > constants::ARENA_HEIGHT - half_rect_height {
                transform.set_y(constants::ARENA_HEIGHT - half_rect_height);
            }
        }
    }
}