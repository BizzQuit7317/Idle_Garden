mod system;
mod overlay;

use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};
use serde::{Serialize, Deserialize};
use crate::subsystems::{Subsystem, SubsystemRegistration, ResourceContext, SubsystemOutput, ItemDefinition};
use crate::systems::player::Property;

#[derive(Debug, Serialize, Deserialize)]
pub struct BedSystem {
    pub soil_quality: u8,
    pub growing_spots: Vec<system::GrowingSpot>,
    pub selected_item: Option<String>, // tracks what the player clicked in inventory
    pub pending_plant: Option<(usize, String)>, // (spot index, item id)
    pub pending_harvest: Option<usize>,
}

impl BedSystem {
    pub fn new() -> BedSystem {
        BedSystem {
            soil_quality: 0,
            growing_spots: vec![system::GrowingSpot::new(); 9],
            selected_item: None,
            pending_plant: None,
            pending_harvest: None,
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
    ($id:expr, $display:expr, $desc:expr, $cash:expr, $conservation:expr) => {
        inventory::submit!(ItemDefinition {
            id: $id,
            display_name: $display,
            description: $desc,
            cash_value: $cash,
            conservation_value: $conservation,
        });
    }
}

register_item!("blue_flower_seed", "Blue Flower Seed", "A small seed for a blue flower.", 1.0, 20.0);
register_item!("flower_flower", "Blue Flower", "A blue flower.", 2.0, 3.0);
register_item!("grass_seeds", "Grass Seeds", "Grass seeds, what else?.", 1.0, 2.0);
register_item!("grass", "Grass", "Regular degular grass buddy.", 5.0, 6.0);