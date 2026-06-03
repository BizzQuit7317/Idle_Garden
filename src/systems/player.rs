use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Property {
    Balcony,
    Terrace,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub property: Property
}

impl Player {
    pub fn new() -> Player {
        Player {
            property: Property::Balcony, //Set the default new user to have the Balcony house
        }
    }
}