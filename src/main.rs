extern crate amethyst;

mod components;
mod constants;
mod states;
mod systems;
mod utils;
mod events;

use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        DisplayConfig, 
        DrawFlat2D, 
        Pipeline,
        RenderBundle, 
        Stage,
        ColorMask,
        ALPHA
    },
    ui::{
        DrawUi,
        UiBundle
    },
    utils::application_root_dir,
};


fn main() -> amethyst::Result<()> {
    use crate::states::GameplayState;

    amethyst::start_logger(Default::default());

    // Display config
    let path = format!(
        "{}/resources/display_config.ron",
        application_root_dir()
    );
    let config = DisplayConfig::load(&path);
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.01, 0.01, 0.01, 1.0], 1.0)
            .with_pass(DrawFlat2D::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                None
            ))
            .with_pass(DrawUi::new())
    );
    // Input config
    let binding_path = format!(
        "{}/resources/bindings_config.ron",
        application_root_dir()
    );
    let input_bundle = InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(
                pipe, 
                Some(config)
            ).with_sprite_sheet_processor()
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<String, String>::new())?;
    let mut game = Application::new("./", GameplayState::new(), game_data)?;

    game.run();

    Ok(())
}
