use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

use crate::systems;
use crate::subsystems::ResourceContext;

#[derive(Debug, Serialize, Deserialize)]
pub struct  GameState {
    pub player: systems::player::Player,
    pub meta: systems::save_meta::SaveMeta,
    pub tick_rate: f64, //seconds per tick
    #[serde(skip, default)] //Sets tick accumulator to 0.0 everytime it gets saved
    pub tick_accumulator: f64, //How much time has built up
    pub max_offline_time: f64, //Maximum time in seconds
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            player: systems::player::Player::new(),
            meta: systems::save_meta::SaveMeta::new(),
            tick_rate: 1.0, //default tick rate of 1 per second
            tick_accumulator: 0.0,
            max_offline_time: 172800.0, //Default for now to 2 days in seconds of offline time
        }
    }

    pub fn process_offline_progress(&mut self) {
        let mut seconds_passed: f64 = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - self.meta.last_saved_at) as f64;
        if seconds_passed > self.max_offline_time {
            seconds_passed = self.max_offline_time
        }
        let ticks_passed: u64 = (seconds_passed / self.tick_rate) as u64;
        for _ in 0..ticks_passed {
            self.tick();
        }
        println!("[DBG]Player was away for {} seconds", seconds_passed);
    }

    pub fn tick(&mut self) {
        let ctx = ResourceContext {
            cash: self.player.cash,
            conservation_points: self.player.conservation_points,
            inventory: self.player.inventory.items.clone(),
        };

        for slot in self.player.slots.iter_mut() {
            if let Some(subsystem) = slot {
                let output = subsystem.tick(&ctx);
                self.player.cash += output.cash_delta;
                self.player.conservation_points += output.conservation_delta;
                for (item, amount) in output.items_produced {
                    self.player.inventory.add(&item, amount);
                }
                for (item, amount) in output.items_consumed {
                    self.player.inventory.remove(&item, amount);
                }
            }
        }

        self.meta.total_ticks += 1 //Always end with this as it just counts up total times this has run
    }
}