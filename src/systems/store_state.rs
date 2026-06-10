use serde::{Serialize, Deserialize};
use crate::subsystems::{available_items, get_item_definition};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreItem {
    pub item_id: String,
    pub price: f64,
    pub quantity_available: u32,
    pub conservation_price: f64,
    pub in_store: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Store {
    #[serde(skip)] //Don't neeed to save the catalogue only need to build it on load
    catalogue: Vec<StoreItem>,
    pub stock: Vec<StoreItem>,
    pub stock_limit: u32,
}

impl Store {
    pub fn new() -> Self {
        let mut store = Store {
            catalogue: vec![],
            stock: vec![],
            stock_limit: 9,
        };
        store.build_catalogue();
        store.build_stock();
        store
    }

    pub fn build_catalogue(&mut self) {
        self.catalogue = available_items()
            .into_iter()
            .map(|def| StoreItem {
                item_id: def.id.to_string(),
                price: def.cash_value,        // placeholder — we'll add base prices to ItemDefinition later
                quantity_available: 5,
                conservation_price: def.conservation_value * 100.0, //makeing the conservation buy price 100 times higher than its sell price
                in_store: def.in_store,
            })
            .collect();
    }

    pub fn build_stock(&mut self) {
        self.stock = self.catalogue.iter().filter(|item| item.price > 0.0 && item.in_store).cloned().collect(); //TESTING ONLY SHOWINGS SEEDS FOR NOW
        self.stock.truncate(self.stock_limit as usize);
    }

    pub fn try_buy(&mut self, item_index: usize, player_funds: f64, use_conservation: bool) -> f64 {
        let price = if let Some(item) = self.stock.get_mut(item_index) {
            let cost = if use_conservation { item.conservation_price } else { item.price };
            if player_funds >= cost {
                item.quantity_available -= 1;
                cost
            } else {
                println!("[DBG]Could not afford");
                return 0.0;
            }
        } else {
            println!("[DBG]Could not afford");
            return 0.0;
        };

        println!("[DBG]Bought item");
        price
    }
}