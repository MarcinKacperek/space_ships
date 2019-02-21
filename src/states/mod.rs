mod load;
mod main_menu;
mod gameplay;
mod pause;
mod result;

pub use {
    load::LoadingState,
    gameplay::GameplayState,
    pause::PauseState
};