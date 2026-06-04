use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

use crate::data::constants;

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveMeta {
    pub created_at: u64,        // unix timestamp, when they first started this save
    pub last_saved_at: u64,     // used to calculate offline progress on load
    pub total_ticks: u64,       // good proxy for total time played, avoids clock manipulation
    pub save_version: u32,      // invaluable when you change the data structure later
}

impl SaveMeta {
    pub fn new() -> SaveMeta {
        SaveMeta {
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(), //Take the currently generated time
            last_saved_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            total_ticks: 0,
            save_version: constants::GAME_VERSION,
        }
    }
}