use serde::{Serialize, Deserialize};

use crate::subsystems::Subsystem;

#[derive(Debug, Serialize, Deserialize)]
pub enum Property {
    Balcony,
    Terrace,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub property: Property,
    pub max_slots: usize,
    pub slots: Vec<Option<Box<dyn Subsystem>>>,

    pub cash: f64,
    pub conservation_points: f64,
}

impl Player {
    pub fn new() -> Player {
        Player {
            property: Property::Balcony, //Set the default new user to have the Balcony house
            max_slots: 2,
            slots: vec![None, None], 

            cash: 0.0, //Start with no money
            conservation_points: 0.0, //Start with no conservation
        }
    }
}