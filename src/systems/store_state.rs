use serde::{Serialize, Deserialize};
use crate::subsystems::{available_items, get_item_definition};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreItem {
    pub item_id: String,
    pub price: f64,
    pub quantity_available: u32,
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
                price: 1.0,        // placeholder — we'll add base prices to ItemDefinition later
                quantity_available: 1,
            })
            .collect();
    }

    pub fn build_stock(&mut self) {
        self.stock = self.catalogue.clone();
        self.stock.truncate(self.stock_limit as usize);
    }
}