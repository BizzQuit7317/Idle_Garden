use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufWriter};
use rand::Rng;

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
            if self.items[item] == 0 {
                self.items.remove(item);
            }
            true
        } else {
            false
        }
    }

    pub fn total_items(&self) -> u64 {
        self.items.len() as u64  // number of distinct types, not total quantity
    }
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
pub enum Property {
    Balcony,
    Terrace,
    Semi,
    Detached,
    Farm,
    Forest,
    Continent,
}

impl Property {
    pub fn upgrade_cost(&self) -> (f64,f64) { //(cash, conservation)
        match self {
            Property::Balcony => (1000.0, 5000.0), 
            Property::Terrace => (5000.0, 20000.0),
            Property::Semi => (10000.0, 50000.0),
            Property::Detached => (50000.0, 100000.0),
            Property::Farm => (1000000.0, 1000000.0),
            Property::Forest => (2000000.0, 5000000.0),
            Property::Continent => (10000000.0, 20000000.0),
        }
    }

    pub fn next(&self) -> Option<Property> {
        match self {
            Property::Balcony => Some(Property::Terrace),
            Property::Terrace => Some(Property::Semi),
            Property::Semi => Some(Property::Detached),
            Property::Detached => Some(Property::Farm),
            Property::Farm => Some(Property::Forest),
            Property::Forest => Some(Property::Continent),
            Property::Continent => None,
        }
    }

    pub fn max_slots(&self) -> usize {
        match self {
            Property::Balcony => 2, 
            Property::Terrace => 4,
            Property::Semi => 6,
            Property::Detached => 8,
            Property::Farm => 10,
            Property::Forest => 12,
            Property::Continent => 15,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub property: Property,
    pub max_slots: usize,
    pub slots: Vec<Option<Box<dyn Subsystem>>>,

    pub cash: f64,
    pub conservation_points: f64,

    pub cash_current_rebirth: f64,
    pub conservation_points_current_rebirth: f64,

    pub cash_mult: f64,
    pub conservation_mult: f64,

    pub inventory: Inventory,

    pub family_name: String,
    pub first_name:  String,
    pub generation: u8,
}

impl Player {
    pub fn new() -> Player {
        let mut inventory = Inventory::new();
        inventory.add("grass_seeds", 5); //Starting the player with 5 grass seeds
        inventory.add("small_cage", 1); //testing the feeder

        let property = Property::Balcony;
        let max_slots = property.max_slots();

        let mut player = Player {
            property, //Set the default new user to have the Balcony house
            max_slots,
            slots: (0..max_slots).map(|_| None).collect(), 

            cash: 0.0,//5000.0, //Start with no money
            conservation_points: 0.0, //Start with no conservation

            cash_current_rebirth: 0.0,
            conservation_points_current_rebirth: 0.0,

            cash_mult: 1.0, //Multiplier on base cash
            conservation_mult: 1.0, //Multiplier for base conservaation

            inventory,

            family_name: String::new(),
    	    first_name:  String::new(),
    	    generation: 0,
        };

	player.pick_name();
	player
    }

    pub fn upgrade_property(&mut self) -> bool {
        if let Some(next) = self.property.next() {
            let (cash_cost, conservation_cost) = self.property.upgrade_cost();
            if self.cash >= cash_cost && self.conservation_points >= conservation_cost {
                self.cash -= cash_cost;
                self.conservation_points -= conservation_cost;
                self.property = next;
                self.max_slots = self.property.max_slots();
                // grow the slots vec to match
                while self.slots.len() < self.max_slots {
                    self.slots.push(None);
                }
                return true;
            }
        }
        false
    }

    pub fn pick_name(&mut self) {
        let file = match File::open("src/systems/assets/names.txt") {
            Ok(f) => f,
            Err(_) => return, // file missing — leave name as-is
        };
        let reader = io::BufReader::new(file);
        let mut rng = rand::thread_rng();
        let mut count: u64 = 0;

    	for line in reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(_) => continue, // skip unreadable line
            };
            if line.trim().is_empty() {
                continue;
            }
            count += 1;
            if rng.gen_range(0..count) == 0 {
                self.first_name = line.trim().to_string();
            }
        }
    }

    pub fn rebirth_calculation(&self) -> (f64, f64) { //(cash mult, cons mult)
        let cons_mult_buffer: f64 = 1.0; //By default will return 1.0 multiplyer
        let cash_mult_buffer: f64 = 1.0; //By default will return 1.0 multiplyer

        let new_cash_mult: f64 = 1.0 + (self.cash_current_rebirth / cash_mult_buffer).sqrt();
        let new_cons_mult: f64 = 1.0 + (self.conservation_points_current_rebirth / cons_mult_buffer).sqrt();

        (new_cash_mult, new_cons_mult)
    }

    pub fn rebirth(&mut self) {
        let (new_cash_mult, new_cons_mult) = self.rebirth_calculation();

        self.cash_mult = new_cash_mult;
        self.conservation_mult = new_cons_mult;

        self.pick_name();
        self.generation += 1;

        self.slots = (0..self.max_slots).map(|_| None).collect();

        self.cash_current_rebirth = 0.0;
        self.conservation_points_current_rebirth = 0.0;

        self.cash = 0.0;
        self.conservation_points = 0.0;

        //Give the player more seeds and a birdcage lol could be a potential exploit to do a shit load of rebirths to max on bird cages and sell them before pregoressing lol
        self.inventory.add("grass_seeds", 5); //Starting the player with 5 grass seeds
        self.inventory.add("small_cage", 1); //testing the feeder
    }

}