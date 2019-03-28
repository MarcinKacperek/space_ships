#[derive(Clone)]
pub enum GameState {
    Running,
    Paused,
    Finished
}

pub struct GameplayNextState {
    pub next_state: Option<GameState>
}