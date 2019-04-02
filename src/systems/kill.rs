use amethyst::{
    core::{
        Transform,
        Time
    },
    ecs::{
        Entities,
        Entity,
        Join,
        WriteExpect,
        WriteStorage,
        Read,
        ReadExpect,
        ReadStorage,
        System
    },
    renderer::{
        SpriteRender,
        SpriteSheetHandle
    }
};
use crate::{
    components::{
        Expire,
        Killable,
        Rect,
        tags::{
            DeleteEntityTag,
            HealthPickupTag,
            PlayerShipTag
        }
    },
    resources::{
        GameplayNextState,
        GameplaySessionData,
        GameState
    }
};

pub struct KillSystem;

impl KillSystem {
    fn drop_pickup<'s>(
        killable_entity: Entity,
        expires: &mut WriteStorage<'s, Expire>,
        health_pickup_tags: &mut WriteStorage<'s, HealthPickupTag>,
        rects: &mut WriteStorage<'s, Rect>,
        sprite_renders: &mut WriteStorage<'s, SpriteRender>,
        transforms: &mut WriteStorage<'s, Transform>,
        entities: &Entities<'s>,
        sprite_sheet_handle: SpriteSheetHandle,
        time: &Read<'s, Time>
    ) {
        // Drop pickup
        let killable_transform = transforms.get(killable_entity).unwrap();
        let mut pickup_transform = Transform::default();
        pickup_transform.set_xyz(
            killable_transform.translation().x, 
            killable_transform.translation().y, 
            0.0
        );

        entities
            .build_entity()
            .with(pickup_transform, transforms)
            .with(
                Rect {
                    width: 32.0,
                    height: 30.0
                },
                rects
            )
            .with(
                Expire::new(5.0, time.absolute_real_time_seconds()),
                expires
            )
            .with(
                SpriteRender {
                    sprite_sheet: sprite_sheet_handle,
                    sprite_number: 18
                },
                sprite_renders
            )
            .with(HealthPickupTag, health_pickup_tags)
            .build();
    }
}

impl<'s> System<'s> for KillSystem {
    type SystemData = (
        ReadStorage<'s, PlayerShipTag>,
        WriteStorage<'s, Expire>,
        WriteStorage<'s, DeleteEntityTag>,
        WriteStorage<'s, HealthPickupTag>,
        WriteStorage<'s, Killable>,
        WriteStorage<'s, Rect>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transform>,
        WriteExpect<'s, GameplayNextState>,
        WriteExpect<'s, GameplaySessionData>,
        Entities<'s>,
        ReadExpect<'s, SpriteSheetHandle>,
        Read<'s, Time>
    );

    fn run(
        &mut self, 
        (
            player_ship_tags, 
            mut expires,
            mut delete_entity_tags,
            mut health_pickup_tags,
            mut killables, 
            mut rects,
            mut sprite_renders,
            mut transforms,
            mut gameplay_next_state, 
            mut session_data, 
            entities,
            sprite_sheet_handle,
            time
        ): Self::SystemData
    ) {
        for (killable, entity) in (&mut killables, &entities).join() {
            if !delete_entity_tags.contains(entity) && !killable.is_alive() {
                if !player_ship_tags.contains(entity) {
                    session_data.score += killable.get_points();
                    if killable.is_drops_health() {
                        KillSystem::drop_pickup(
                            entity,
                            &mut expires,
                            &mut health_pickup_tags,
                            &mut rects,
                            &mut sprite_renders,
                            &mut transforms,
                            &entities,
                            sprite_sheet_handle.clone(),
                            &time
                        );
                    }
                } else {
                    gameplay_next_state.next_state = Some(GameState::Finished);
                }

                let _ = delete_entity_tags.insert(entity, DeleteEntityTag);
            }
        }
    }
}