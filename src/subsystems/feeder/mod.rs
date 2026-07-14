pub mod system;
mod overlay;

use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};
use serde::{Serialize, Deserialize};
use crate::subsystems::{Subsystem, SubsystemRegistration, ResourceContext, SubsystemOutput, ItemDefinition};
use crate::systems::player::Property;
use std::collections::HashMap;
use crate::systems::npc::NPC;
use crate::systems::popup::Modal;

#[derive(Debug, Serialize, Deserialize)]
pub struct FeederSystem {
    pub npc_id: String,
    pub feeder_level: u32,
    pub upgrade_price: f64,
    pub current_feeder: Option<system::FeederDefinition>,
    pub current_food: Option<system::FoodDefinition>,
    pub current_food_amount: u32,
    pub current_birds: Vec<(system::BirdDefinition, f64)>, //(bird, ticker)
    pub bird_pool: Vec<(system::BirdDefinition, f64)>, //(bird, spawn weight)
    pub pending_feeder: Option<system::FeederDefinition>,
    pub pending_food: Option<system::FoodDefinition>,
    pub pending_items: HashMap<String, u32>,
    pub selected_item: Option<String>, // tracks what the player clicked in inventory
    pub feeder_definitions: Vec<system::FeederDefinition>,
    pub food_definitions: Vec<system::FoodDefinition>,
    pub bird_definitions: Vec<system::BirdDefinition>,
    pub decay_rate_ticker: f64,
    pub spawn_ticker: f64,
    pub dropped_items: HashMap<String, u32>,
    #[serde(skip)]
    pub pending_modals: Vec<Modal>,
    pub pending_upgrade: bool,
}

impl FeederSystem {
    pub fn new() -> FeederSystem {
        FeederSystem {
            npc_id: String::from("feeder_npc"),
            feeder_level: 1,
            upgrade_price: 100.0,
            current_feeder: None,
            current_food: None,
            current_food_amount: 0,
            current_birds: vec![],
            bird_pool: vec![],
            pending_feeder: None,
            pending_food: None,
            pending_items: HashMap::new(),
            selected_item: None,
            feeder_definitions: system::load_feeder_definitions(),
            food_definitions: system::load_food_definitions(),
            bird_definitions: system::load_bird_definitions(),
            decay_rate_ticker: 0.0,
            spawn_ticker: 10.0, //base cooldown
            dropped_items: HashMap::new(),
            pending_modals: vec![],
            pending_upgrade: false,
        }
    }
}

#[typetag::serde]
impl Subsystem for FeederSystem {
    fn tick(&mut self, ctx: &ResourceContext) -> SubsystemOutput {
        system::tick(self, ctx)
    }

    fn name(&self) -> &str { "feeder" }

    fn draw_overlay(&mut self, ui: &mut macroquad::ui::Ui, ctx: &ResourceContext) {
        overlay::draw(ui, self, ctx);
    }
}

inventory::submit!(SubsystemRegistration {
    create: || Box::new(FeederSystem::new()),
    min_property: Property::Balcony,
});

macro_rules! register_item {
    ($id:expr, $display:expr, $desc:expr, $cash:expr, $conservation:expr, $store:expr) => {
        inventory::submit!(ItemDefinition {
            id: $id,
            display_name: $display,
            description: $desc,
            cash_value: $cash,
            conservation_value: $conservation,
            in_store: $store,
        });
    }
}

register_item!("bird_feed", "Bird Feed", "Seeds and scraps left by visiting birds.", 15.0, 1.0, true);
register_item!("feather", "Feather", "A small feather left behind by a visiting bird.", 1.0, 1.0, false);
register_item!("small_cage", "Small Cage Feeder", "A small cage feeder", 1.0, 1.0, false);
register_item!("medium_cage", "Medium Cage Feeder", "A medium cage feeder", 1.0, 1.0, false);
register_item!("platform", "Platform Feeder", "A flat platform feeder open to all birds", 1.0, 1.0, false);
register_item!("worm", "Worm", "A juicy worms birds and fish can't resist", 1.0, 1.0, false);