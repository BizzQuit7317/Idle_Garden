mod system;
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
pub struct BedSystem {
    pub npc_id: String,
    pub bed_level: u32,
    pub growing_spots: Vec<system::GrowingSpot>,
    pub selected_item: Option<String>, // tracks what the player clicked in inventory
    pub pending_plant: Option<(usize, String)>, // (spot index, item id)
    pub pending_harvest: Option<usize>,
    pub pending_waste: Option<usize>,
    pub pending_fertilise: Option<(usize, String)>,
    pub plant_definitions: Vec<system::PlantDefinition>,
    pub fertiliser_definitions: Vec<system::FertiliserDefinition>,
    pub auto_harvested_items: HashMap<String, u32>,
    #[serde(skip)]
    pub pending_modals: Vec<Modal>,
    pub pending_upgrade: bool,
    pub upgrade_price: f64,
}

impl BedSystem {
    pub fn new() -> BedSystem {
        BedSystem {
            npc_id: String::from("bed_npc"),
            bed_level: 1,
            growing_spots: vec![system::GrowingSpot::new(); 1], //Player starts with 1 by defaults
            selected_item: None,
            pending_plant: None,
            pending_harvest: None,
            pending_waste: None,
            pending_fertilise: None,
            plant_definitions: system::load_plant_definitions(),
            fertiliser_definitions: system::load_fertiliser_definitions(),
            auto_harvested_items: HashMap::new(),
            pending_modals: vec![],
            pending_upgrade: false,
            upgrade_price: 100.0, //base upgrade
        }
    }
}

#[typetag::serde]
impl Subsystem for BedSystem {
    fn tick(&mut self, ctx: &ResourceContext) -> SubsystemOutput {
        system::tick(self, ctx)
    }

    fn name(&self) -> &str { "bed" }

    fn draw_overlay(&mut self, ui: &mut macroquad::ui::Ui, ctx: &ResourceContext) {
        overlay::draw(ui, self, ctx);
    }
}

inventory::submit!(SubsystemRegistration {
    create: || Box::new(BedSystem::new()),
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

// Tier 1 - Grass
register_item!("grass_seeds", "Grass Seeds", "Grass seeds, what else?", 1.0, 1.0, true);
register_item!("grass", "Grass", "Regular degular grass buddy.", 3.0, 1.0, false);

// Tier 2 - Blue Flower
register_item!("blue_flower_seed", "Blue Flower Seed", "A small seed for a blue flower.", 5.0, 2.0, true);
register_item!("blue_flower", "Blue Flower", "A pretty blue flower.", 12.0, 2.0, false);

// Tier 3 - Sunflower
register_item!("sunflower_seed", "Sunflower Seed", "A big cheerful seed.", 15.0, 3.0, true);
register_item!("sunflower", "Sunflower", "Tall and bright.", 35.0, 3.0, false);

// Tier 4 - Rose
register_item!("rose_seed", "Rose Seed", "A delicate seed.", 45.0, 5.0, true);
register_item!("rose", "Rose", "A beautiful rose.", 100.0, 5.0, false);

// Tier 5 - Orchid
register_item!("orchid_seed", "Orchid Seed", "A rare and fussy seed.", 130.0, 8.0, true);
register_item!("orchid", "Orchid", "An exotic orchid.", 300.0, 8.0, false);

// Seeds from birds - Pumpkin
register_item!("pumpkin_seed", "Pumpkin Seed", "A seed that grows a pumpkin.", 130.0, 8.0, false);
register_item!("pumpkin", "Pumpkin", "A Pumpkin.", 300.0, 8.0, false);

// Utility
register_item!("mulch", "Mulch", "Plant waste.", 0.0, 2.0, false);
register_item!("compost", "Compost", "Helps seeds sprout and establish faster.", 8.0, 3.0, false);
register_item!("plant_feed", "Plant Feed", "A liquid feed that speeds up all growth stages.", 25.0, 5.0, false);
register_item!("growth_powder", "Growth Powder", "Makes plants grow quicker at all stages.", 100.0, 200.0, false);