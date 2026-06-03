use serde::{Serialize, Deserialize};

use crate::data;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub housing: data::constants::Houses
}

impl Player {
    pub fn new() -> Player {
        Player {
            housing: data::constants::Houses::Balcony, //Set the default new user to have the Balcony house
        }
    }

    pub fn to_page(&self) -> data::constants::Page {
        match &self.housing {
            data::constants::Houses::Balcony => data::constants::Page::Balcony,
            data::constants::Houses::Terrace => data::constants::Page::Default,
        }
    }
}