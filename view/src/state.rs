use game::*;

pub struct GameState {
    pub players: Vec<Player>,
    pub width: f32,
    pub height: f32,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            players: Vec::new(),
            width: 1280.0,
            height: 800.0,
        }
    }
}