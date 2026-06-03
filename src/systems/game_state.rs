use serde::{Serialize, Deserialize};

use crate::systems;

#[derive(Debug, Serialize, Deserialize)]
pub struct  GameState {
    pub player: systems::player::Player,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            player: systems::player::Player::new(),
        }
    }

    pub fn tick(&mut self) {
        println!("Running the game logic")
    }
}