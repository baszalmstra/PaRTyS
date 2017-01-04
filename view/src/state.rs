use player;

pub struct State {
    pub players: Vec<player::Player>,
    pub width: f32,
    pub height: f32,
}

impl State {
    pub fn new() -> State {
        State {
            players: Vec::new(),
            width: 1920.0,
            height: 1080.0,
        }
    }
}