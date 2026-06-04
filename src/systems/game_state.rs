use serde::{Serialize, Deserialize};

use crate::systems;

#[derive(Debug, Serialize, Deserialize)]
pub struct  GameState {
    pub player: systems::player::Player,
    pub meta: systems::save_meta::SaveMeta,
    pub tick_rate: f64, //seconds per tick
    pub tick_accumulator: f64, //How much time has built up
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            player: systems::player::Player::new(),
            meta: systems::save_meta::SaveMeta::new(),
            tick_rate: 1.0, //default tick rate of 1 per second
            tick_accumulator: 0.0,
        }
    }

    pub fn tick(&mut self) {
        self.player.cash += 1.0;



        self.meta.total_ticks += 1 //Always end with this as it just counts up total times this has run
    }
}