use amethyst::{
    ecs::{
        ReadExpect,
        System
    },
    renderer::SpriteSheetHandle
};
use crate::components::data::{
    GameplaySessionData,
    UiAssets,
    UiGameplayElements
};

pub struct GameplayUiSystem;

// impl<'s> System<'s> for GameplayUiSystem {
//     type SystemData = (
//         ReadExpect<'s, GameplaySessionData>,
//         ReadExpect<'s, UiAssets>,
//         ReadExpect<'s, UiGameplayElements>,
//         ReadExpect<'s, SpriteSheetHandle>
//     );
// }