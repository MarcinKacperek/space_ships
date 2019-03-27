use amethyst::core::Transform;
use crate::components::Rect;
 
pub fn is_aabb_collide(
    first_rect: &Rect,
    first_transform: &Transform,
    second_rect: &Rect,
    second_transform: &Transform
) -> bool {
    let first_left = first_transform.translation().x - first_rect.width / 2.0;
    let first_right = first_left + first_rect.width;
    let first_bottom = first_transform.translation().y - first_rect.height / 2.0;
    let first_top = first_bottom + first_rect.height;

    let second_left = second_transform.translation().x - second_rect.width / 2.0;
    let second_right = second_left + second_rect.width;
    let second_bottom = second_transform.translation().y - second_rect.height / 2.0;
    let second_top = second_bottom + second_rect.height;

    return
        first_left <= second_right &&
        first_right >= second_left &&
        first_bottom <= second_top &&
        first_top >= second_bottom;
}