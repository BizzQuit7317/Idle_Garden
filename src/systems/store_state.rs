use serde::{Serialize, Deserialize};
use crate::subsystems::{available_items, get_item_definition, ItemRole};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreItem {
    pub item_id: String,
    pub price: f64,
    pub quantity_available: u32,
    pub conservation_price: f64,
    pub in_store: bool,
    pub item_role: ItemRole,
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
            .filter(|def| def.in_store)
            .map(|def| StoreItem {
                item_id: def.id.to_string(),
                price: def.cash_value,        // placeholder — we'll add base prices to ItemDefinition later
                quantity_available: 5,
                conservation_price: def.conservation_value * 100.0, //makeing the conservation buy price 100 times higher than its sell price
                in_store: def.in_store,
                item_role: def.item_role,
            })
            .collect();
    }

    pub fn build_stock(&mut self) {
        /*
            When building stock store has a limit of 9 slots
            It should use at minimum 4 core but maximum 3 utility and 2 boost
            If no items for boost or utility availlable just use core
            If all items available do a rng check to see if they spawn
            Boost should be the rarest and Utility the middle rarity
        */
        let mut core_buffer: Vec<StoreItem> = self.catalogue.iter().filter(|item| item.item_role == ItemRole::Core).cloned().collect();
        let mut utility_buffer: Vec<StoreItem> = self.catalogue.iter().filter(|item| item.item_role == ItemRole::Utility).cloned().collect();
        let mut boost_buffer: Vec<StoreItem> = self.catalogue.iter().filter(|item| item.item_role == ItemRole::Boost).cloned().collect();

        core_buffer.shuffle(&mut thread_rng());
        core_buffer.truncate(self.stock_limit as usize); //Keep max size incase nothing else gets selected

        utility_buffer.shuffle(&mut thread_rng());
        utility_buffer.truncate(3); //3 is maximum number of utility items shop can have

        boost_buffer.shuffle(&mut thread_rng());
        boost_buffer.truncate(2); //2 is maximum number of boost items shop can have

        let mut rng = thread_rng();
        utility_buffer.retain(|_| rng.gen_bool(0.5));
        boost_buffer.retain(|_| rng.gen_bool(0.25));

        let mut final_buffer: Vec<StoreItem> = vec![]; //When adding buffers must add core buffer last
        final_buffer.extend(utility_buffer);
        final_buffer.extend(boost_buffer);
        final_buffer.extend(core_buffer); 

        final_buffer.truncate(self.stock_limit as usize); //Haveing core last means we dont need any addtional logic
        self.stock = final_buffer;
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