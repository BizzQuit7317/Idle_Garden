use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::subsystems::Subsystem;

#[derive(Debug, Serialize, Deserialize)]
pub struct Inventory {
    pub items: HashMap<String, u64>,
    pub capacity: u64,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            items: HashMap::new(),
            capacity: 9,
        }
    }

    pub fn add(&mut self, item: &str, amount: u64) -> bool {
        let is_new_item = !self.items.contains_key(item);
        let would_exceed = is_new_item && self.items.len() >= self.capacity as usize;
        
        if would_exceed {
            false
        } else {
            *self.items.entry(item.to_string()).or_insert(0) += amount;
            true
        }
    }

    pub fn remove(&mut self, item: &str, amount: u64) -> bool {
        let current = self.items.get(item).copied().unwrap_or(0);
        if current >= amount {
            *self.items.get_mut(item).unwrap() -= amount;
            true
        } else {
            false
        }
    }

    pub fn total_items(&self) -> u64 {
        self.items.len() as u64  // number of distinct types, not total quantity
    }
}

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

    pub inventory: Inventory,
}

impl Player {
    pub fn new() -> Player {
        let mut inventory = Inventory::new();
        inventory.add("grass_seed", 5); //Starting the player with 5 grass seeds

        Player {
            property: Property::Balcony, //Set the default new user to have the Balcony house
            max_slots: 2,
            slots: vec![None, None], 

            cash: 0.0, //Start with no money
            conservation_points: 0.0, //Start with no conservation

            inventory,
        }
    }
}