use amethyst::{
    core::{
        Parent,
        Transform
    },
    ecs::{
        Entities,
        Join,
        ReadExpect,
        ReadStorage,
        System,
        WriteStorage,
        world::Index
    },
    ui::{
        Anchor,
        UiImage,
        UiText,
        UiTransform
    }
};
use crate::{
    components::{
        Killable,
        Rect,
        tags::{
            PlayerShipTag
        },
        ui::UiKillable
    },
    resources::{
        GameplaySessionData,
        UiAssets,
        UiGameplayElements
    }
};

pub struct UiSystem;    

impl UiSystem {

    fn update_score<'s>(
        gameplay_session_data: &ReadExpect<'s, GameplaySessionData>,
        ui_gameplay_elements: &ReadExpect<'s, UiGameplayElements>,
        ui_texts: &mut WriteStorage<'s, UiText>
    ) {
        if let Some(text) = ui_texts.get_mut(ui_gameplay_elements.score_value_text) {
            text.text = gameplay_session_data.score.to_string();
        }
    }

    fn update_player_lives<'s>(
        player_ship_tags: &ReadStorage<'s, PlayerShipTag>,
        killables: &mut WriteStorage<'s, Killable>,
        ui_texts: &mut WriteStorage<'s, UiText>,
        ui_gameplay_elements: &ReadExpect<'s, UiGameplayElements>
    ) {
        for (player_killable, _) in (killables, player_ship_tags).join() {
            if let Some(text) = ui_texts.get_mut(ui_gameplay_elements.life_value_text) {
                text.text = player_killable.get_health().to_string();
            }
        }
    }

    fn update_health_bars<'s>(
        transforms: &ReadStorage<'s, Transform>,
        rects: &ReadStorage<'s, Rect>,
        player_ship_tags: &ReadStorage<'s, PlayerShipTag>,
        killables: &mut WriteStorage<'s, Killable>,
        parents: &mut WriteStorage<'s, Parent>,
        ui_images: &mut WriteStorage<'s, UiImage>,
        ui_killables: &mut WriteStorage<'s, UiKillable>,
        ui_transforms: &mut WriteStorage<'s, UiTransform>,
        entities: &mut Entities<'s>,
        ui_assets: &ReadExpect<'s, UiAssets>
    ) {
        // TODO Too long, too confusing, refactor
        for (killable, transform, rect, _) in (killables, transforms, rects, !player_ship_tags).join() {
            let health_bar_entity = if let Some(health_bar_entity_index) = killable.health_bar_entity_index {
                let health_bar_entity = entities.entity(health_bar_entity_index);
                let ui_killable = ui_killables.get_mut(health_bar_entity).unwrap();
                if killable.get_health() != ui_killable.last_health {
                    let mut i = 0;
                    for health_segment_index in &ui_killable.health_segment_entities {
                        i += 1;
                        if i > killable.get_health() {
                            let health_segment_entity = entities.entity(*health_segment_index);
                            let ui_image = ui_images.get_mut(health_segment_entity).unwrap();
                            ui_image.texture = ui_assets.get_health_bar_red_img();
                        }
                    }

                    ui_killable.last_health = killable.get_health();
                }

                health_bar_entity
            } else {
                let health_bar_transform = UiTransform::new(
                    String::from(""),
                    Anchor::BottomLeft,
                    0.0,
                    0.0,
                    1.0,
                    rect.width,
                    10.0,
                    1
                );
                let health_bar_border_image = UiImage {
                    texture: ui_assets.get_health_bar_border_img()
                };
                let health_bar_entity = entities
                    .build_entity()
                    .with(health_bar_transform, ui_transforms)
                    .with(health_bar_border_image, ui_images)
                    .build();

                let mut health_segment_entities: Vec<Index> = Vec::new();
                
                let health_segment_width = (rect.width - 2.0 - 2.0 * killable.get_max_health() as f32) / killable.get_max_health() as f32;
                let mut health_segment_x = 2.0 + health_segment_width / 2.0;
                for i in 0..killable.get_max_health() {
                    let health_bar_segment_transform = UiTransform::new(
                        String::from(""),
                        Anchor::MiddleLeft,
                        health_segment_x,
                        0.0,
                        1.0,
                        health_segment_width,
                        8.0,
                        1
                    );
                    let health_bar_segment_image = UiImage {
                        texture: {
                            if i < killable.get_health() {
                                ui_assets.get_health_bar_green_img()
                            } else {
                                ui_assets.get_health_bar_red_img()
                            }
                        }
                    };
                    let health_bar_segment_entity = entities
                        .build_entity()
                        .with(Parent { entity: health_bar_entity }, parents)
                        .with(health_bar_segment_transform, ui_transforms)
                        .with(health_bar_segment_image, ui_images)
                        .build();
                    health_segment_entities.push(health_bar_segment_entity.id());

                    health_segment_x = health_segment_x + health_segment_width + 2.0;
                }

                let _ = ui_killables.insert(health_bar_entity, UiKillable {
                    last_health: killable.get_health(),
                    health_segment_entities: health_segment_entities
                });
                killable.health_bar_entity_index = Some(health_bar_entity.id());

                health_bar_entity
            };

            let ui_transform = ui_transforms.get_mut(health_bar_entity).unwrap();
            ui_transform.local_x = transform.translation().x;
            ui_transform.local_y = transform.translation().y + rect.height / 2.0 + 10.0;
        }
    }

    // Handle ammo bar

}

impl<'s> System<'s> for UiSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Rect>,
        ReadStorage<'s, PlayerShipTag>,
        WriteStorage<'s, Killable>,
        WriteStorage<'s, Parent>,
        WriteStorage<'s, UiImage>,
        WriteStorage<'s, UiKillable>,
        WriteStorage<'s, UiText>,
        WriteStorage<'s, UiTransform>,
        Entities<'s>,
        ReadExpect<'s, GameplaySessionData>,
        ReadExpect<'s, UiAssets>,
        ReadExpect<'s, UiGameplayElements>
    );

    fn run(
        &mut self,
        (
            transforms,
            rects,
            player_ship_tags,
            mut killables,
            mut parents,
            mut ui_images,
            mut ui_killables,
            mut ui_texts,
            mut ui_transforms,
            mut entities,
            gameplay_session_data,
            ui_assets,
            ui_gameplay_elements
        ): Self::SystemData 
    ) {
        UiSystem::update_player_lives(
            &player_ship_tags, 
            &mut killables, 
            &mut ui_texts, 
            &ui_gameplay_elements
        );
        UiSystem::update_score(
            &gameplay_session_data, 
            &ui_gameplay_elements,
            &mut ui_texts
        );
        UiSystem::update_health_bars(
            &transforms,
            &rects,
            &player_ship_tags,
            &mut killables,
            &mut parents,
            &mut ui_images,
            &mut ui_killables,
            &mut ui_transforms,
            &mut entities,
            &ui_assets
        );
    }

}